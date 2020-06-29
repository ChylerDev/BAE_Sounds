#[cfg(test)]
mod tests {
    use bae_sounds::*;

    use bae_gen::*;
    use bae_mod::*;
    use bae_types::*;
    use bae_utils::*;

    use std::fs::File;
    use std::sync::Arc;
    use std::time::Duration;

    const SAMPLE_RATE: usize = 48_000;

    #[test]
    fn test_blocks() {
        let mut b = BaeBlock::from_generator(Sine::new(440.0, SAMPLE_RATE as Math));
        let mut s = Sine::new(440.0, SAMPLE_RATE as Math);

        for _ in 0..seconds_to_samples(
            std::time::Duration::from_secs_f64(1.0 / 440.0),
            SAMPLE_RATE as Math,
        ) {
            assert!((b.process() - s.process()).abs() < 1e-15);
        }

        let mut b = BaeBlock::from_modifier(LowPass::new(440.0, 1.0, SAMPLE_RATE as Math));
        let mut n = Noise::new();

        let mut t = SamplerackT::new();

        for _ in 0..seconds_to_samples(Duration::from_secs_f64(0.5), SAMPLE_RATE as Math) {
            b.prime_input(n.process());
            t.push(b.process());
        }

        normalize_write(
            -1.5,
            t,
            &mut File::create(".junk/sounds/block_NoiseLP.wav").unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn test_bae_sounds() {
        let mut ss = BaeSound::new(
            1.0,
            0.5,
            Arc::new(BaeBlock::from_generator(Noise::new())),
        );
        ss.extend_modifiers(vec![
            Arc::new(BaeBlock::from_modifier(LowPass::new(
                440.0,
                1.0,
                SAMPLE_RATE as Math,
            ))),
            Arc::new(BaeBlock::from_modifier(HighPass::new(
                220.0,
                1.0,
                SAMPLE_RATE as Math,
            ))),
        ]);

        let mut t = SamplerackT::new();

        for _ in 0..seconds_to_samples(Duration::from_secs(4), SAMPLE_RATE as Math) {
            t.push(ss.process(0.0));
        }

        normalize_write(
            -1.5,
            t,
            &mut File::create(".junk/sounds/simple_sounds.wav").unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn test_complex_sounds() {
        let mut cs = ComplexSound::new(1.0, 1.0);

        let n = cs.add_block(Arc::new(BaeBlock::from_generator(Noise::new())));
        let lp = cs.add_block(Arc::new(BaeBlock::from_modifier(LowPass::new(
            440.0,
            1.0,
            SAMPLE_RATE as Math,
        ))));
        let hp = cs.add_block(Arc::new(BaeBlock::from_modifier(HighPass::new(
            220.0,
            1.0,
            SAMPLE_RATE as Math,
        ))));

        cs.add_connection(cs.get_input_gain(), n);
        cs.add_connection(n, lp);
        cs.add_connection(lp, hp);
        cs.add_connection(hp, cs.get_output_gain());

        let mut t = SamplerackT::new();

        for _ in 0..seconds_to_samples(Duration::from_secs(4), SAMPLE_RATE as Math) {
            t.push(cs.process(0.0));
        }

        normalize_write(
            -1.5,
            t,
            &mut File::create(".junk/sounds/complex_sounds.wav").unwrap(),
        )
        .unwrap();
    }

    fn normalize_write(
        db: Math,
        mut t: SamplerackT,
        d: &mut dyn std::io::Write,
    ) -> Result<(), ()> {
        normalize(db, &mut t);

        WaveWriteOptions::new()
            .bps(24)?
            .r(SAMPLE_RATE as Math)
            .clip(true)
            .write(vec![t], d)
            .expect("Failed to write wav file");

        Ok(())
    }
}
