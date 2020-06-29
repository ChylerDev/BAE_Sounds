//! # BaeBlock
//!
//! A structure implementing the ability to create a graph of different
//! [`Generator`]s, [`Modifier`]s, and the form of interaction between them to
//! create large-scale effects such as those found in synthesizers.
//!
//! [`Generator`]: ../../generators/trait.Generator.html
//! [`Modifier`]: ../../modifiers/trait.Modifier.html

use super::*;

use bae_gen::*;
use bae_mod::*;
use std::sync::Arc;

type GeneratorSP = Arc<dyn Generator>;
type ModifierSP = Arc<dyn Modifier>;

/// Type defining the closure that combines inputted Sample samples from the
/// outputs of the [`Generator`]s and [`Modifier`]s of the containing
/// [`BaeBlock`].
///
/// [`Generator`]: ../../generators/trait.Generator.html
/// [`Modifier`]: ../../modifiers/trait.Modifier.html
/// [`BaeBlock`]: struct.BaeBlock.html
pub type InterBase = dyn FnMut(Sample, Sample) -> Sample;

/// Reference-counted wrapper for the closure [`InterBase`]
///
/// [`InterBase`]: type.InterBase.html
pub type Inter = Arc<InterBase>;

/// Struct used for generalizing the structure of and abstracting the [`Sound`]
/// struct. This allows us to create complex sounds as a graph of [`BaeBlock`]s,
/// where each block can be a [`Modifier`], [`Generator`], or both, and there output
/// of the [`BaeBlock`] is defined as some user-definable combination of the
/// [`Generator`] and [`Modifier`] output. See [`Sound`] documentation for more info.
///
/// Internally, the [`Generator`], [`Modifier`], and [`Inter`] are stored wrapped
/// within an [`Arc`]. This means that when you clone a [`BaeBlock`], the
/// internal objects are *not* cloned. Rather, their reference count is incremented,
/// and the wrapped objects stay where they are.
///
/// [`Generator`]: ../../generators/trait.Generator.html
/// [`Modifier`]: ../../modifiers/trait.Modifier.html
/// [`BaeBlock`]: struct.BaeBlock.html
/// [`Sound`]: struct.Sound.html
/// [`Inter`]: type.Inter.html
/// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
pub struct BaeBlock {
    g: GeneratorSP,
    m: ModifierSP,
    i: Inter,
    input: Sample,
}

impl BaeBlock {
    /// Creates a new BaeBlock from the given [`Generator`], [`Modifier`], and
    /// [`Inter`].
    ///
    /// # Parameters
    ///
    /// * `g` - The [`Generator`] for the [`BaeBlock`].
    /// * `m` - The [`Modifier`] for the [`BaeBlock`].
    /// * `i` - The interactor (typically a closure) that defines the combination
    /// of `g`s and `m`s samples when `BaeBlock::process()` is called.
    ///
    /// [`Generator`]: ../../generators/trait.Generator.html
    /// [`Modifier`]: ../../modifiers/trait.Modifier.html
    /// [`BaeBlock`]: struct.BaeBlock.html
    /// [`Inter`]: type.Inter.html
    pub fn new<T, U>(g: T, m: U, i: Inter) -> Self
    where
        T: 'static + Generator,
        U: 'static + Modifier,
    {
        BaeBlock {
            g: Arc::new(g),
            m: Arc::new(m),
            i,
            input: Sample::default(),
        }
    }

    /// Creates a new block from the given [`Generator`]. For the [`BaeBlock`],
    /// [`Empty`] is used for the `m`, and the return value of
    /// [`BaeBlock::generator_passthrough`] is used for `i`.
    ///
    /// # Parameters
    /// * `g` - The [`Generator`] for the [`BaeBlock`].
    ///
    /// [`Generator`]: ../../generators/trait.Generator.html
    /// [`BaeBlock`]: struct.BaeBlock.html
    /// [`BaeBlock::generator_passthrough`]: struct.BaeBlock.html#method.generator_passthrough
    /// [`Inter`]: type.Inter.html
    /// [`Empty`]: ../../generators/empty/struct.Empty.html
    pub fn from_generator<T>(g: T) -> Self
    where
        T: 'static + Generator,
    {
        BaeBlock {
            g: Arc::new(g),
            m: Arc::new(Passthrough::new()),
            i: Self::generator_passthrough(),
            input: Sample::default(),
        }
    }

    /// Creates a new block from the given [`Modifier`]. For the [`BaeBlock`],
    /// [`Empty`] is used for `g`, and the return value of
    /// [`BaeBlock::modifier_passthrough`] is used for `i`.
    ///
    /// # Parametrs
    /// * `m` - The [`Modifier`] for the [`BaeBlock`].
    ///
    /// [`Modifier`]: ../../modifiers/trait.Modifier.html
    /// [`BaeBlock`]: struct.BaeBlock.html
    /// [`BaeBlock::modifier_passthrough`]: struct.BaeBlock.html#method.modifier_passthrough
    /// [`Inter`]: type.Inter.html
    /// [`Empty`]: ../../modifiers/empty/struct.Empty.html
    pub fn from_modifier<U>(m: U) -> Self
    where
        U: 'static + Modifier,
    {
        BaeBlock {
            g: Arc::new(Zero::new()),
            m: Arc::new(m),
            i: Self::modifier_passthrough(),
            input: Sample::default(),
        }
    }

    /// Creates the default interactor which simply multiplies the two passed
    /// samples together.
    pub fn default_interactor() -> Inter {
        Arc::new(|ge, mo| ge * mo)
    }

    /// Creates a passthrough interactor which passes the [`Generator`] sample
    /// through.
    ///
    /// [`Generator`]: ../../generators/trait.Generator.html
    pub fn generator_passthrough() -> Inter {
        Arc::new(|ge, _| ge)
    }

    /// Creates a passthrough interactor which passes the [`Modifier`] sample
    /// through.
    ///
    /// [`Modifier`]: ../../modifiers/trait.Modifier.html
    pub fn modifier_passthrough() -> Inter {
        Arc::new(|_, mo| mo)
    }

    /// Returns a reference to the [`Generator`] wrapped in a smart pointer.
    ///
    /// [`Generator`]: ../../generators/trait.Generator.html
    pub fn get_g(&self) -> &GeneratorSP {
        &self.g
    }

    /// Returns a reference to the [`Modifier`] wrapped in a smart pointer.
    ///
    /// [`Modifier`]: ../../modifiers/trait.Modifier.html
    pub fn get_m(&self) -> &ModifierSP {
        &self.m
    }

    /// Returns a mutable reference to the [`Generator`] wrapped in a smart pointer.
    ///
    /// [`Generator`]: ../../generators/trait.Generator.html
    pub fn get_g_mut(&mut self) -> &mut GeneratorSP {
        &mut self.g
    }

    /// Returns a mutable reference to the [`Modifier`] wrapped in a smart pointer.
    ///
    /// [`Modifier`]: ../../modifiers/trait.Modifier.html
    pub fn get_m_mut(&mut self) -> &mut ModifierSP {
        &mut self.m
    }
}

impl Block for BaeBlock {
    fn prime_input(&mut self, x: Sample) {
        self.input += x;
    }

    fn process(&mut self) -> Sample {
        let y = Inter::get_mut(&mut self.i).unwrap()(
            GeneratorSP::get_mut(&mut self.g).unwrap().process(),
            ModifierSP::get_mut(&mut self.m)
                .unwrap()
                .process(self.input),
        );

        self.input = Sample::default();

        y
    }
}

/// Alias for a [`BaeBlock`] object wrapped in a smart pointer.
///
/// [`BaeBlock`]: struct.BaeBlock.html
pub type BaeBlockSP = Arc<BaeBlock>;
