//! # BaeSound
//!
//! Module containing types implementing the ability to run a single
//! [`Generator`] and [`Modifier`] within a single object, granting the ability
//! for fast processing of simple [`Generator`]s and [`Modifier`]s into a single
//! output.
//!
//! [`Generator`]: ../../generators/trait.Generator.html
//! [`Modifier`]: ../../generators/trait.Modifier.html

use super::*;

/// Struct implementing the ability to run a single [`Generator`] through a
/// given list of [`Modifier`]s operated in series. This allows for simple and
/// fast processing of the structure's elements while still allowing for a wide
/// range of more complex sounds.
///
/// [`Generator`]: ../../generators/trait.Generator.html
/// [`Modifier`]: ../../modifiers/trait.Modifier.html
#[derive(Clone)]
pub struct BaeSound {
    generator: BlockSP,
    modifier_list: Vec<BlockSP>,
    input_gain: Sample,
    output_gain: Sample,
    id: Option<usize>,
    is_muted: bool,
    is_paused: bool,
}

impl BaeSound {
    /// Constructs a new [`BaeSound`] object. The new object is initialized
    /// with an empty [`Vec`] of [`Modifier`]s. Add [`Modifier`]s with
    /// [`add_modifier`] or [`extend_modifiers`].
    ///
    /// [`BaeSound`]: struct.BaeSound.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [`Modifier`]: ../../modifiers/trait.Modifier.html
    /// [`add_modifier`]: struct.BaeSound.html#method.add_modifier
    /// [`extend_modifiers`]: struct.BaeSound.html#method.extend_modifiers
    pub fn new(input_gain: Math, output_gain: Math, generator: BlockSP) -> Self {
        BaeSound {
            generator,
            modifier_list: Vec::new(),
            input_gain: input_gain as Sample,
            output_gain: output_gain as Sample,
            id: None,
            is_muted: false,
            is_paused: false,
        }
    }

    /// Adds a single modifier to the internal [`Vec`] of [`Modifier`]s.
    ///
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [`Modifier`]: ../../modifiers/trait.Modifier.html
    pub fn add_modifier<M>(&mut self, m: BlockSP)
    where
        M: 'static + Clone,
    {
        self.modifier_list.push(m);
    }

    /// Extends the internal [`Vec`] of [`Modifier`]s with the given [`Vec`].
    ///
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
    /// [`Modifier`]: ../../modifiers/trait.Modifier.html
    pub fn extend_modifiers(&mut self, m_list: Vec<BlockSP>) {
        self.modifier_list.extend(m_list);
    }

    /// Returns the linear gain applied to the input during processing.
    pub fn get_input_gain(&self) -> Math {
        self.input_gain as Math
    }

    /// Returns the linear gain applied to the output during processing.
    pub fn get_output_gain(&self) -> Math {
        self.output_gain as Math
    }

    /// Sets the input linear gain that is applied during processing.
    pub fn set_input_gain(&mut self, g: Math) {
        self.input_gain = g as Sample;
    }

    /// Sets the output linear gain that is applied during processing.
    pub fn set_output_gain(&mut self, g: Math) {
        self.output_gain = g as Sample;
    }
}

impl Sound for BaeSound {
    fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    fn is_paused(&self) -> bool {
        self.is_paused
    }

    fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted
    }

    fn is_muted(&self) -> bool {
        self.is_muted
    }

    fn register(&mut self, id: usize) {
        self.id = Some(id);
    }

    fn unregister(&mut self) {
        self.id = None;
    }

    fn process(&mut self, input: Sample) -> Sample {
        if self.is_paused {
            return Default::default();
        }

        let mut out = if let Some(b) = BlockSP::get_mut(&mut self.generator) {
            b.prime_input(input * self.input_gain);
            b.process()
        } else {
            Default::default()
        };

        for m in &mut self.modifier_list {
            if let Some(m) = BlockSP::get_mut(m) {
                m.prime_input(out);
                out = m.process();
            }
        }

        if self.is_muted {
            Default::default()
        } else {
            out * self.output_gain
        }
    }

    fn get_id(&self) -> Option<usize> {
        self.id
    }
}
