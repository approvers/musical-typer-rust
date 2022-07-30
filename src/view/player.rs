use rich_sdl2_rust::mixer::{
  chunk::{channel::ChannelGroup, MixChunk},
  device::MixDevice,
  music::MixMusic,
};
use rich_sdl2_rust::SdlError;
use std::{collections::HashMap, path::Path};
use PlayerError::*;

pub enum SEKind {
  Correct,
  Fail,
  Vacant,
  GameOver,
  MissedSentence,
  PerfectSentence,
  PerfectSection,
}

#[derive(Debug)]
pub enum PlayerError {
  AudioError(SdlError),
  FileError(std::io::Error),
}

impl From<std::io::Error> for PlayerError {
  fn from(err: std::io::Error) -> Self {
    FileError(err)
  }
}

type Chunks<'a> = HashMap<String, MixChunk<'a>>;

pub struct Player<'music> {
  device: &'music MixDevice<'music>,
  music: Option<MixMusic<'music>>,
  chunks: Chunks<'music>,
  group: ChannelGroup<'music>,
}

impl<'music> Player<'music> {
  pub fn new(device: &'music MixDevice) -> Self {
    Self {
      device,
      music: None,
      chunks: load_chunks(device).expect("missing audio file dir"),
      group: ChannelGroup::new(device, 40),
    }
  }

  pub fn change_bgm(
    &mut self,
    bgm_name: &str,
  ) -> Result<(), PlayerError> {
    let bgm_file_path = format!("score/{}", bgm_name);
    let music = MixMusic::new(self.device, &bgm_file_path)
      .map_err(AudioError)?;
    self.music = Some(music);
    self.play_bgm()?;
    Ok(())
  }

  pub fn play_bgm(&self) -> Result<(), PlayerError> {
    if let Some(ref music) = self.music {
      music.play(Some(1)).map_err(AudioError)?;
    }
    Ok(())
  }

  pub fn stop_bgm(&self, fade_time: u32) -> Result<(), PlayerError> {
    if let Some(ref music) = self.music {
      music.fade_out(fade_time).map_err(AudioError)?;
    }
    Ok(())
  }

  fn play_se_file(&self, name: &str) -> Result<(), PlayerError> {
    let chunk =
      self.chunks.get(name).expect("sound effect not loaded");
    if let Some(channel) = self.group.first_free() {
      channel
        .play(chunk, Default::default())
        .map_err(AudioError)?;
      Ok(())
    } else {
      Err(PlayerError::AudioError(SdlError::Others {
        msg: "not found free channel".into(),
      }))
    }
  }

  pub fn play_se(&self, kind: SEKind) -> Result<(), PlayerError> {
    use SEKind::*;
    match kind {
      Correct => self.play_se_file("correct"),
      Fail => self.play_se_file("fail"),
      Vacant => self.play_se_file("vacant"),
      GameOver => self.play_se_file("gameover"),
      MissedSentence => self.play_se_file("missed"),
      PerfectSentence => self.play_se_file("perfect_sentence"),
      PerfectSection => self.play_se_file("perfect_section"),
    }
  }
}

fn load_chunks<'music>(
  device: &'music MixDevice,
) -> Result<Chunks<'music>, PlayerError> {
  let path = Path::new("asset/");
  let mut chunks: Chunks = HashMap::new();
  for entry in path.read_dir()? {
    let file = entry?;
    if file.path().extension().map_or(false, |ext| ext == "wav") {
      chunks.insert(
        file.path().file_stem().map_or("".into(), |name| {
          name.to_string_lossy().to_string()
        }),
        {
          let chunk = MixChunk::new(
            device,
            file.path().to_str().expect("invalid chunk path"),
          )
          .map_err(AudioError)?;
          chunk.set_volume(112);
          chunk
        },
      );
    }
  }
  Ok(chunks)
}
