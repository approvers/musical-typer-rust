use crate::view::{handler::MouseState, Component};
use rich_sdl2_rust::{color::Rgba, geo::Rect, renderer::pen::Pen};

#[derive(PartialEq)]
pub struct ButtonProps {
  pub border_color: Rgba,
  pub color_on_hover: Rgba,
  pub mouse: MouseState,
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

    if self
      .bounds
      .contains_point(self.props.mouse.started_pressing)
      && self.bounds.contains_point(self.props.mouse.ended_pressing)
    {
      (self.on_click)();
    }
  }

  fn render(&self, pen: &Pen<'_>) {
    let &Button { props, bounds, .. } = &self;
    let &ButtonProps {
      color_on_hover,
      border_color,
      mouse,
    } = &props;

    let on_hover = bounds.contains_point(mouse.mouse_pos);

    if on_hover {
      pen.set_color(color_on_hover);
      pen.fill_rect(bounds);
    }

    pen.set_color(border_color);
    pen.stroke_rect(bounds);
  }
}
