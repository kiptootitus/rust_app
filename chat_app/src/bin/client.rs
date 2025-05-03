#! [allow(warnings)]
// Importing the various modules from the cursive Library for the UI dev.
use cursive::{
  align::HAlign,
  event::Key,
  theme::{BorderStyle, Palette, Theme, Color, PaletteColor, BaseColor},
  traits::*,
  views::{Dialog, DummyView, EditView, ScrollView, TextView, LinearLayout, Panel},
  Cursive,
};

use tokio::{
  io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
  net::TcpStream,
  sync::Mutex,
};

use chrono::Local;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

}