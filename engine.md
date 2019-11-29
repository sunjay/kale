# Rendering Engine Architecture

This document contains the implementation design for the new rendering engine
catered specifically to the needs of the turtle crate.

## Requirements

- [ ] Draws all the rendering primitives supported by turtle:
  - [ ] lines
  - [ ] arcs, circles
  - [ ] cuves (e.g. bezier)
  - [ ] filled shapes made out of arbitrary combinations of lines, arcs, etc.
    - [ ] ability to change the fill color during the course of the animation
  - [ ] text
    - [ ] animated text drawing (letter by letter)
  - [ ] images
- [ ] Supports arbitrary window / viewport size (e.g. for dynamic resizing)
- [ ] Single, consistent drawing order
  - [ ] Multiple overlapping shapes that are both being filled should still have
    a consistent draw order (e.g. the one with the first begin_fill is drawn below)
  - With multiple turtles, we don't ever want one of the turtles to always draw
    over another turtle or something like that
  - The drawing order should be: newer shapes should be drawn on top of older
    shapes (ideally, we'd do this at the pixel level, but that is too hard)
- [ ] Allows adjacent lines to be joined when the pen is the same
  - Goal: Avoid cases like this: ([Source](https://github.com/sunjay/turtle/pull/100))
    ![](https://user-images.githubusercontent.com/3689561/46027483-91387f00-c10b-11e8-8bbb-f530b035eaa8.png)
  - Note that automatically joining the lines means that line capping can make
    the image not be totally consistent in between lines (need to evaluate impact of this)
    - e.g. a thick line switching from being capped to being part of another line may not look good
    - See also: [LineJoin](https://docs.rs/lyon_tessellation/0.14.2/lyon_tessellation/enum.LineJoin.html)
  - Adjacent lines from different turtles should not be joined
- [ ] Configurable [line cap style]: butt, square, round
- [ ] Arbitrary insertion, replacement or deletion from the drawing queue
  - To support undo/redo at both a turtle level, and a drawing level
  - To allow for clear at both a turtle level, and a drawing level
- [ ] Mutation of filled shapes wherever they are in the drawing queue
  - So we can continue to add more points to a filled shape as we draw
  - Need to keep the filled shape underneath the lines that surround it even as we add points
- [ ] Does **not** store turtle state
  - Renderer should only deal with drawing things as fast as possible and managing
    the drawing queue
- [ ] Lazy re-rendering by default
  - Renderer should not re-render more than necessary (e.g. stop re-rendering if
    nothing in the queue has changed)
  - Renderer should not re-render static items in the queue more than necessary
    - e.g. if everything in the queue before a certain point will never change,
      there is no point in re-rendering that stuff over and over again. We should
      cache the framebuffer and blit it onto the screen.
  - Renderer should try to avoid re-rendering on resize (e.g. if everything fits
    within the new window size

[line cap style]: https://docs.rs/lyon_tessellation/0.14.2/lyon_tessellation/enum.LineCap.html

## Implementation Design: Overview

Though the renderer shouldn't be designed with much knowledge of the turtle
crate embedded in its design, it is important to start thinking from the turtle
crate so we can ensure that this will actually work for the crate. We want to
avoid tight coupling, while still making something useful.

The overall flow, starting from the turtle crate, will look approximately as
follows:

1. Main Process: The methods on `Turtle` like `forward`, `backward`, `left`,
   `right`, etc. will be called
2. Main process: This will prompt the creation of a rendering primitive and its
   animation data
3. Main process: The rendering primitive is tessellated using the lyon crate
4. Main process: The tessellated primitive is placed on the drawing queue and
   marked as dynamic because it is still being animated
   * An ID to this newly added queue item is stored in the main process so we can
     continuously update this shape
5. Renderer process: Handles drawing the tessellated primitive, taking into account
   that it is dynamic and will change
6. Main Process: As time goes by, the tessellated primitive is updated over and
   over again to forward the animation

The original renderer for turtle serialized and deserialized data in order to
get things between the processes. This is highly inefficient, especially given
the large volume of data we need to process. It is much more efficient to use a
shared memory based solution.

### Enforcing Drawing Order

Scenario that makes this very hard: Suppose you have N turtles, all placed
in a circle around the origin. Each turtle faces in the origin. Each turtle
starts at a different distance from the origin, then proceeds to draw a line
of arbitrary length towards the origin. Each turtle will cross the origin
at some point, and then continue animating the same line for an undetermined
amount of time. If each turtle has a distinct line color, is there a way to
order the lines so that they appear at the origin in the order in which they
cross the origin? This is very hard to do with only a single primitive
somewhere in the drawing queue. By using a drawing queue design, the order
of the lines will all appear in the order in which they were added to the
queue. This is completely independent of the order in which they cross the
origin.

Solving this problem in a very general way is very difficult because it's almost
impossible to simulate every pixel of a line's animation and end up putting the
right pixels in the right places in the image.

We will likely end up just going with the optimal but (occasionally) unintuitive
behaviour of relying on the order in which drawings are added to the queue.
Doing this at the pixel level just isn't feasible.

### Rendering Graphics Primitives

After tesselation, we need to render the shapes generated by the lyon crate. To
do this, we'll likely use a crate like [luminance](https://docs.rs/luminance).
This crate provides mid-level abstractions that should be good enough for our
use case.

We'll want to render static items from the drawing queue into a separate frame
buffer object so we can quickly blit that onto the screen and redraw anything
dynamic on top.

### Adjoining Adjacent Lines

If we want to adjoin adjacent lines with the same pen, we may run into issues
because it's harder to effectively cache renders for static shapes. For example,
suppose that an entire drawing is made with a single pen. As each part of the
drawing finishes animating, it is marked as static. The renderer can then
attempt to cache the rendered version of that part. Unfortunately, because
adjacent lines are automatically joined, we have to throw away the cached render
every time as we adjoin more lines.

The solution here might be to just cap all lines with a round cap and not adjoin
adjacent lines at all. Another alternative would be to limit the number of
adjoined lines to a reasonable number that we can render fairly quickly. The
potential problem with that is that there may be visible differences between the
adjoined lines and lines that are just adjacent but not actually drawn together.
That means that we'd get "gaps" or weird artifacts showing up every few lines.

We might want to consider supporting some "smart" automatic adjoining that takes
into account how visible the adjoining would be. We can potentially calculate
that using the width of the line and the angle between the lines.

## Code Structure

* `DrawingQueue` - shared memory used by the renderer process and the main thread
* `Renderer`
  * manages drawing items in the queue
  * manages drawing cache for static primitives
  * the owner of the drawing queue
