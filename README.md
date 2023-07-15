# Mike's Cube Problem Attempt

The problem is based off of Computerphile's video on the subject: https://www.youtube.com/watch?v=g9n0a0644B4

The problem is known as the "Enumerating Polycube" problem. The video explains it better than I can, so I recommend watching it.

The problem is as follows:
There is a combination of 3D cubes that make a shape. Each cube must have a face touching another cube. This means that diagonals do not count. Shapes are to be unique, meaning that if a shape is rotated and it is a shape that was already made. Since this is 3D, we are working with 3 axes. The possible rotations in 3D are:

- On the X axis
    - 0 degrees
    - 90 degrees
    - 180 degrees
    - 270 degrees
- On the Y axis
    - 0 degrees
    - 90 degrees
    - 180 degrees
    - 270 degrees
- On the Z axis
    - 0 degrees
    - 90 degrees
    - 180 degrees
    - 270 degrees

Now there are corresponding negative rotations, but those are not needed since they are the same as the positive rotations. This means that there are 12 possible rotations for each shape.

The goal is to find all the possible shapes that can be made with a given number of cubes.

## Possible Problems

 - This is a problem of time vs space. It is a balance of trying to use as little space as possible, but also trying to use as little time as possible. This is because the more space you use, the more time it takes to search through all the possible shapes. The more time you use, the more space you use to store all the shapes.

   - You could think of it in terms like so. We could store all the shapes in a list, but that would take up a lot of space. Now using that space and looking up will incur a time penalty as well. Now if we were to generate the shapes as we go, we would not have to store them, but we would have to generate them every time we want to check if a shape is unique. This would incur a much larger time penalty, but would virtually take up no space.


## Attempt 1

### Method

We are going to use Rust to solve this problem. I am using it for the following reasons:

- It is a compiled language, meaning that it will be faster than an interpreted language.
- It is a low level language, meaning that it will be faster than a high level language.
- It is a systems language, meaning that it will be faster than a language that is not a systems language.
- It is a language I would like to get better at.

I am planning on seeing if I could add a visual component to this problem. Maybe to see the shapes as they are being generated. I am not sure if I will be able to do this, but I will try.