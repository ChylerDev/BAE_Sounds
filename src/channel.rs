//! # Channels
//!
//! This module includes the types and traits needed to process a group of
//! [`Sound`]s to generate a single output sample.
//!
//! [`Sound`]: https://docs.rs/bae_sounds/0.1.0/trait.Sound.html

use super::*;

use bae_sf::*;
use std::sync::Arc;
use std::time::Duration;

pub mod bae_channel;
pub use bae_channel::*;

type SoundSP = Arc<dyn Sound>;

/// Trait defining the simplest possible interface for a channel, with the
/// ability to process a batch of samples at a time.
pub trait Channel<SF>
where
    SF: SampleFormat,
{
    /// Sets the amount of time [`process`] should calculate samples for. The
    /// given duration is truncated to a integer sample value.
    ///
    /// [`process`]: trait.Channel.html#tymethod.process
    fn set_process_time(&mut self, d: Duration);

    /// Returns a reference to the internal track of samples.
    fn get_output(&self) -> &Vec<SF>;

    /// Sets the gain of the output of the channel.
    fn set_gain(&mut self, gain: Math);

    /// Processes the given number of samples, storing the results in the
    /// internal track of samples.
    fn process(&mut self);

    /// Adds a [`Sound`] to the [`Channel`] for processing.
    ///
    /// [`Channel`]: trait.Channel.html
    /// [`Sound`]: https://docs.rs/bae_sounds/0.1.0/trait.Sound.html
    fn add_sound(&mut self, sound: &mut SoundSP);

    /// Removes a [`Sound`] from the [`Channel`].
    ///
    /// The `id` parameter can be accessed from the registered [`Sound`] itself.
    ///
    /// [`Channel`]: trait.Channel.html
    /// [`Sound`]: https://docs.rs/bae_sounds/0.1.0/trait.Sound.html
    fn remove_sound(&mut self, id: usize);
}
