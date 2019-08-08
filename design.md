# design notes

- [ ] Straight lines
- [ ] Arcs
- [ ] Circles
- [ ] Filled version of a shape made from any of the above (single fill color)
- [ ] Border of filled shape may change its drawing properties at any point along the path
- [ ] Text
- [ ] Images (stamps?)
- [ ] Multiple turtles
- [ ] Undo/Redo

```rust
fn main() {
  let mut turtle = Turtle::new();

  turtle.set_pen_size(5.0);
  turtle.set_pen_color(color::RED);
  turtle.set_fill_color(color::CYAN);

  turtle.begin_fill();
  turtle.forward(10.0);
  turtle.right(10.0);

  // Change of pen halfway through, still filling
  turtle.set_pen_color(color::BLUE);
  turtle.forward(10.0);
  // counterclockwise
  turtle.arc(
    10.0, // radius left of the turtle
    30.0, // degrees of the circle to draw
  );
}
```

## Line Capping

Rules:

* Pen down when turtle is created
* Pen down => line is capped at start
* Pen up => line is capped at end
* Otherwise => line is not capped at the end
* Pen down then pen up => line is not capped at the end

```rust
struct Pen {
  pub enabled: bool,
  pub color: Color,
  pub stroke: StrokeOptions,
}

impl Default for Pen {
  fn default() -> Self {
    Self {
      enabled: true,
      color: color::BLACK,
      stroke: StrokeOptions {
        start_cap: LineCap::Round,
        line_width: 1.0,
        ..StrokeOptions::default()
      },
    }
  }
}

struct Turtle {
  /// The current path animated by the turtle
  animated_path: Path,
}

impl Turtle {
  pub fn new() -> Self {
    Self {
    }
  }
}
```

## Multiple Turtles

Multiple turtles need to have the right drawing order. How does overlap work?

```rust
fn main() {
  let mut drawing = turtle::Drawing::new();

  let turtle1 = drawing.add_turtle();
  let turtle2 = drawing.add_turtle();

  turtle1.set_pen_color(color::RED);
  turtle1.pen_up();
  // bottom left corner
  turtle1.go_to(-10.0, -10.0);
  // turn towards top right
  turtle1.right(45.0);
  turtle2.pen_down();

  turtle1.set_pen_color(color::BLUE);
  turtle2.pen_up();
  // bottom right corner
  turtle2.go_to(10.0, -10.0);
  // turn towards top left
  turtle2.left(45.0);
  turtle2.pen_down();

  // Draw towards each other
  turtle1.forward(100.0);
  turtle2.forward(100.0);
}
```

As the turtle lines overlap and move past each other, how should the red and
blue lines overlap? If we just use the "first come first serve" rule, they will
alternate the overlap back and forth (which would look quite odd). Once we
decide on an overlap order, it shouldn't change. Furthermore, the overlap order
should be decided based on which line reaches the overlap point(s) first, not
just which line is first. That means that there can be multiple (!!!) overlap
points with different behaviours.

## Time

An interesting way to do this would be to exploit the depth buffer in order to
create some notion of time. Instead of starting from `(x1, y1)` and going to
`(x2, y2)`, we would use `(x1, y1, t1)` and `(x2, y2, t2)`. The `t` values would
be what we put into the depth buffer.

We'll use whatever the smallest floating point increment is (e.g. `1e-6` or
whatever). And we can reset `t1` to `0` every time no one is animating anymore.
Just in case, we'll use wrapping arithmetic so there is never any panicking.

The depth buffer is populated from the `z` coordinates of the point returned
from the vertex shader. We would just have to be careful that this isn't
accidentally used in any actual shader calculations or we'll end up with some
incorrect renders.

## More notes on time

To avoid floating point precision problems, it is probably safest to use the
relative time since the start of the program (or some other starting point)
represented as an integer. We can then losslessly convert that integer to a
floating point number. This conversion is lossless as long as the integer is
quite small. From my testing, we can count up to `16777216` without running into
any issues. (`16777217` is represented the same was as `16777216`).

This is proven by the following program:

```rust
fn main() {
    let mut prev = 0.0;
    for i in 10_000_000u128.. {
        let val = i as f32;
        assert_ne!(prev, val, "prev = {}, i = {}, val = {}", prev, i, val);
        prev = val;
    }
}
```

Note that this limit is because we're using `f32`. With 64-bits it would be much
larger.

If we represent the time in milliseconds (good enough for 60 FPS), we can get
`16777216 ms` which is approximately 4 hours and 40 minutes. The program won't
crash after this point. Instead, you'll get a slightly subtler bug:
crisscrossing lines will be drawn with one side always on top.

Given that it takes almost 5 hours of running the program to reach this point,
we could consider this a reasonable trade-off and only switch to something more
robust if people complain.

## More Robust Than Time

A potentially more robust (and much more complex) solution is to keep "overlap
counters" for each pixel. The idea is that we create something akin to the depth
buffer, but instead of using the time that the process has been running to
populate its values, we keep an individual count per pixel. This will probably
involve storing something in each drawing that records the non-zero count values
for each of its pixels (very memory intensive). This will let us populate the
depth buffer by looking up the order of a given pixel in the drawing. Computing
this over and over again would probably be very complex, so storing the values
during the initial drawing of a shape is probably easier.

## Push and Replace

This is a less robust version than the "time" solution, but may be much simpler
to implement. It is a generalization of the notion of a "temporary" path that
we have been using so far.

Essentially, we have two drawing commands:

* `PushPath(Path)` - appends a new path to the display list
* `ReplacePath(Path)` - replaces the last path **placed by this turtle** on the
  display list

The idea is that all turtles share the same display list in the drawing. Instead
of having a "temporary" path that is always drawn on top, we keep a normal
"first-come-first-serve" order for all paths. The turtle keeps a "cursor" to
their last drawn path. This can then be used to update that path if
`ReplacePath` is used.

To cope with undo/redo, the cursor will have to be sufficiently robust so that
it does not get invalidated by updates to the display list. We can also consider
maybe storing the display list as a BTree or something and giving each node a
unique ID. This would save us the trouble of having to re-index everything
whenever an item is deleted.

This is less robust because the pixels will have the drawing order of when they
initially get pushed into the display list. So multiple overlaps will not be
supported. This might result in some unintuitive behaviour, but I think it is a
reasonable trade-off.
