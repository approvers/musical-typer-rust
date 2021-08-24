use super::{
  sections::{section::note::Note, Sections},
  token::{Token, TokenContent},
  Scoremap,
};
use crate::model::exp::time::DurationError;
use processor::{ParserBody, ParserCtx};
use std::collections::VecDeque;

mod processor;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum ScoremapParseError {
  Command {
    line_num: usize,
    reason: &'static str,
  },
  PropertyDefinition {
    line_num: usize,
    reason: &'static str,
  },
  StatementDefinition {
    line_num: usize,
    reason: &'static str,
  },
  TimingDefinition {
    line_num: usize,
    reason: &'static str,
  },
  Duration {
    line_num: usize,
    err: DurationError,
  },
}

struct Parser {
  func: ParserBody,
  next: Option<Box<Parser>>,
}

impl Parser {
  fn new(f: ParserBody) -> Self {
    Self {
      func: f,
      next: None,
    }
  }

  fn connect(self, other: Self) -> Self {
    let next = if let Some(next) = self.next {
      next.connect(other)
    } else {
      other
    };
    Self {
      next: Some(Box::new(next)),
      ..self
    }
  }

  fn parse(
    &self,
    tokens: &mut VecDeque<&Token>,
    ctx: &mut ParserCtx,
  ) -> Option<Result<Note, ScoremapParseError>> {
    (self.func)(tokens, ctx).or_else(|| {
      self.next.as_ref().and_then(|next| next.parse(tokens, ctx))
    })
  }
}

pub fn parse(
  tokens: &[Token],
) -> Result<Scoremap, ScoremapParseError> {
  use processor::*;

  let mut tokens: VecDeque<_> = tokens
    .iter()
    .filter(|t| {
      !matches!(
        t,
        Token {
          content: TokenContent::Comment,
          ..
        }
      )
    })
    .collect();

  let double_time_parser: Parser = Parser::new(double_time_processor);

  let single_time_parser: Parser = Parser::new(single_time_processor);

  let command_parser: Parser = Parser::new(command_processor);

  let caption_parser: Parser = Parser::new(caption_processor);

  let property_parser: Parser = Parser::new(property_processor);

  let yomigana_parser: Parser = Parser::new(yomigana_processor);

  let section_parser: Parser = Parser::new(section_processor);

  let lyrics_parser: Parser = Parser::new(lyrics_processor);

  let comment_parser: Parser = Parser::new(comment_processor);

  let parser = double_time_parser
    .connect(single_time_parser)
    .connect(command_parser)
    .connect(caption_parser)
    .connect(property_parser)
    .connect(yomigana_parser)
    .connect(section_parser)
    .connect(lyrics_parser)
    .connect(comment_parser);

  let mut ctx = ParserCtx::new();
  while !tokens.is_empty() {
    if let Some(note) =
      parser.parse(&mut tokens, &mut ctx).transpose()?
    {
      ctx.notes.push(note);
    }
  }
  // 最後に空白ノーツを追加
  if let Some(last_duration) =
    ctx.notes.last().map(|note| note.duration().following(1.0))
  {
    ctx.notes.push(Note::blank(last_duration));
  }
  ctx.sections.push(ctx.notes);

  Ok(Scoremap {
    metadata: ctx.metadata,
    sections: Sections::new(ctx.sections),
  })
}
