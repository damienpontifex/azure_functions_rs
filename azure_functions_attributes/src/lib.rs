#![feature(box_patterns)]
use proc_macro::TokenStream;
mod inputs;
use inputs::{TimerTriggerInputs, QueueTriggerInputs};

mod trigger;

#[proc_macro_attribute]
pub fn timer_trigger(args: TokenStream, item: TokenStream) -> TokenStream {
    trigger::impl_trigger::<TimerTriggerInputs>(args, item, "TimerInfo").into()
}

#[proc_macro_attribute]
pub fn event_grid_trigger(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn blob_storage_trigger(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn notification_hub_trigger(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn queue_trigger(args: TokenStream, item: TokenStream) -> TokenStream {
    trigger::impl_trigger::<QueueTriggerInputs>(args, item, "QueueTrigger").into()
}
