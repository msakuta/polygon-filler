# polygon-filler

A super simple polygon rasterizer in Rust

## How to run

Cui:

    cargo r

Gui:

    cargo r -p gui

## What?

This is a project to explore a fast algorithm to fill inside of a polygon using scanline algorithm.
Note that we don't want to use OpenGL or WebGL.

## Why?

Filling inside a polygon sounds an easy task, but if you want to do it as fast as possible, it is not trivial.
If it was triangle, it can be a little bit simple (GPUs use triangles), but it would require triangulation (converting a polygon into a series of triangles).
I want to try implementing an algorithm without triangulation and see how fast it can be.

## How?

We represent each edge of the polygon in a vector equation:

$$
\vec{x} = t \vec{d} + \vec{p}_0
$$

where $\vec{d} = (x_d, y_d)$ represents the direction of the edge, and $\vec{p}_0 = (x_0, y_0)$ reprensets the position of the starting vertex.

The vector equation for the scanline is:

$$
\vec{x} = s\hat{x} + y\hat{y}
$$

where $\hat{x}, \hat{y}$ reprensents the unit vectors along each axis, $s$ being the parameter along the x axis, and $y$ being the coordinate along Y axis.
Be aware that $s$ is the variable along the scanline, while $y$ is fixed.

Now, we want to find out the intersection of the edge and the scanline to get the starting and ending point along the scanline.
Erasing $\vec{x}$ and rewriting the equations to each of X and Y coordinates:

$$
\begin{cases}
s = t x_d + x_0 \\
y = t y_d + y_0
\end{cases}
$$

Solving these equations, we get this:

$$
\begin{align*}
s &= \frac{y - y_0}{y_d}x_d + x_0 \\
t &= \frac{y - y_0}{y_d}
\end{align*}
$$

These are mapped to Rust functions like these:

```rust
fn get_t(y: f64, v: [f64; 2], d: [f64; 2]) -> f64 {
    (y - v[1]) / d[1]
}

fn get_s(y: f64, v: [f64; 2], d: [f64; 2]) -> f64 {
    (y - v[1]) * d[0] / d[1] + v[0]
}
```

Be aware that if $x_d$ was 0, the equation is indeterminate, because the scanline and the edge are parallel.
It manifests as infinity by dividing by zero.
However, we can skip that case because it won't contribute to the filling of the polygon anyway.
Just don't forget to skip the case:

```rust
if d[0] == 0. {
    continue;
}
```

If the condition $0 < t < l$ holds ($l$ meaning the length of the edge), the intersecting point is on the edge.
The number of intersections among all edges should be always even number, because the scanline enters and leaves the polygon the same number of times.
Sometimes floating points do some trick and give us odd number of intersections, but let's ignore it for now.
Given the even number of intersections, we can fill the inside between $(2n + 1)$-th and $2n$-th intersections to fill concave polygons.

## GUI screencast

![gif animation](https://github.com/msakuta/msakuta.github.io/blob/master/images/showcase/polygon-filler.gif?raw=true)


## Example output

It outputs something like:

```
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
-----------------------------**---------------------------------
---------------------------*****--------------------------------
--------------------------******--------------------------------
-------------------------********-------------------------------
-----------------------***********------------------------------
----------------------*************-----------------------------
---------------------***************----------------------------
-------------------*****************----------------------------
------------------*******************---------------------------
-----------------*********************--------------------------
---------------************************-------------------------
--------------**************************------------------------
-------------***************************------------------------
-----------******************************-----------------------
----------********************************----------------------
--------------*****************************---------------------
------------------**************************--------------------
----------------------**********************--------------------
--------------------------*******************-------------------
------------------------------****************------------------
----------------------------------*************-----------------
--------------------------------------**********----------------
------------------------------------------******----------------
----------------------------------------------***---------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
```

Or you can give `poly` argument and get:

```
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
-----------------------------**---------------------------------
---------------------------*****--------------------------------
--------------------------*******-------------------------------
-------------------------*********------------------------------
-----------------------************-----------------------------
----------------------**************----------------------------
---------------------****************---------------------------
-------------------*******************--------------------------
------------------*********************-------------------------
-----------------***********************------------------------
---------------**************************-----------------------
--------------****************************----------------------
-------------******************************---------------------
-----------*********************************--------------------
----------***********************************-------------------
-----------***********************************------------------
-----------************************************-----------------
------------************************************----------------
------------*************************************---------------
-------------*************************************--------------
-------------******************************---------------------
--------------**********************----------------------------
--------------***************-----------------------------------
---------------*******------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
----------------------------------------------------------------
```