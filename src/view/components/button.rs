use crate::view::Component;
use rich_sdl2_rust::{
  color::Rgb, event::mouse::MouseEvent, geo::Rect, renderer::pen::Pen,
};

pub struct ButtonProps {
  pub border_color: Rgb,
  pub color_on_hover: Rgb,
  pub mouse: MouseEvent,
}

impl PartialEq for ButtonProps {
  fn eq(&self, other: &Self) -> bool {
    self.border_color == other.border_color
      && self.color_on_hover == other.color_on_hover
  }
}

pub struct Button<H> {
  props: ButtonProps,
  bounds: Rect,
  on_click: H,
}

impl<H: FnMut()> Button<H> {
  pub fn new(props: ButtonProps, bounds: Rect, on_click: H) -> Self {
    Self {
      props,
      bounds,
      on_click,
    }
  }
}

impl<H: FnMut()> Component for Button<H> {
  type Props = ButtonProps;

  fn is_needed_redraw(&self, new_props: &Self::Props) -> bool {
    &self.props != new_props
  }

  fn update(&mut self, props: Self::Props) {
    self.props = props;

    if let MouseEvent::Button(button) = self.props.mouse {
      if button.pos.is_in(self.bounds) {
        (self.on_click)();
      }
    }
  }

  fn render(&self, pen: &Pen<'_>) {
    let &Button { props, bounds, .. } = self;
    let &ButtonProps {
      color_on_hover,
      border_color,
      mouse,
    } = &props;

    let on_hover =
      if let MouseEvent::Motion(motion) = self.props.mouse {
        motion.pos.is_in(bounds)
      } else {
        false
      };

    if on_hover {
      pen.set_color(color_on_hover);
      pen.fill_rect(bounds);
    }

    pen.set_color(border_color);
    pen.stroke_rect(bounds);
  }
}
