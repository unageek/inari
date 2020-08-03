## Example command lines

[🐌]: Takes a long time to finish graphing.

```
"sin(x) == cos(y)"
```

```
"y - x == sin(exp(x + y))"
```

```
"(x^2 + y^2) == 1 || y == -cos(x)"
```

From Fig. 1a in Tupper (2001)

```
"y < sqrt(x)"
```

From Fig. 17 in Tupper (2001) [🐌]

```
"y == x - atan(tan(x))"
```

Some intriguing examples from GrafEq

- 📂 Single Relation/Abstract/Simple/
  - 📄 Parabolic Waves.gqs

    ```
    "abs(sin(sqrt(x^2 + y^2))) == abs(cos(x))"
    ```

  - 📄 Pythagorean Pairs.gqs [🐌]

    ```
    "floor(x)^2 + floor(y)^2 == floor(sqrt(floor(x)^2 + floor(y)^2))^2" \
    -b -40 40 -40 40
    ```

  - 📄 Pythagorean Triples.gqs [🐌]

    ```
    "floor(x)^2 + floor(y)^2 == 25"
    ```

- 📂 Single Relation/Abstract/Traditionally Difficult/
  - 📄 Infinite Frequency.gqs

    ```
    "y == sin(40/x)"
    ```

  - 📄 O Spike.gqs

    ```
    "(x*(x - 3)/(x - 3.001))^2 + (y*(y - 3)/(y - 3.001))^2 == 81"
    ```

  - 📄 Solid Disc.gqs

    ```
    "81 - x^2 - y^2 == abs(81 - x^2 - y^2)"
    ```

  - 📄 Spike.gqs

    ```
    "y == x*(x - 3)/(x - 3.001)"
    ```

  - 📄 Step.gqs
    
    ```
    "y == atan(10^309 * (x - 1))" 
    ```

  - 📄 Upper Triangle.gqs

    ```
    "x + y == abs(x + y)"
    ```

  - 📄 Wave.gqs

    ```
    "y == sin(x)/x"
    ```

- 📂 Single Relation/Enumerations/Trees/

  - 📄 bi-infinite binary tree.gqs [🐌]

    ```
    "sin(exp2(floor(y))*x + pi/4*(y - floor(y)) - pi/2) == 0 || \
     sin(exp2(floor(y))*x - pi/4*(y - floor(y)) - pi/2) == 0"
    ```

- 📂 Single Relation/Linelike/
  - 📄 Frontispiece #2.gqs

    ```
    "x/cos(x) + y/cos(y) == x*y/cos(x*y) || x/cos(x) + y/cos(y) == -(x*y/cos(x*y)) || \
     x/cos(x) - y/cos(y) == x*y/cos(x*y) || x/cos(x) - y/cos(y) == -(x*y/cos(x*y))"
    ```

  - 📄 Frontispiece.gqs

    ```
    "x/sin(x) + y/sin(y) == x*y/sin(x*y) || x/sin(x) + y/sin(y) == -(x*y/sin(x*y)) || \
     x/sin(x) - y/sin(y) == x*y/sin(x*y) || x/sin(x) - y/sin(y) == -(x*y/sin(x*y))"
    ```

  - 📄 Hair.gqs [🐌]

    ```
    "sin((x + sin(y)) * (sin(x) + y)) == cos(sin((sin(x) + cos(y)) * (sin(y) + cos(x)))) || \
     sin((x + sin(y)) * (sin(x) + y)) == cos(sin((sin(x) + cos(y)) * (sin(y) - cos(x)))) || \
     sin((x + sin(y)) * (sin(x) + y)) == cos(sin((sin(x) - cos(y)) * (sin(y) + cos(x)))) || \
     sin((x + sin(y)) * (sin(x) + y)) == cos(sin((sin(x) - cos(y)) * (sin(y) - cos(x)))) || \
     sin((x + sin(y)) * (sin(x) - y)) == cos(sin((sin(x) + cos(y)) * (sin(y) + cos(x)))) || \
     sin((x + sin(y)) * (sin(x) - y)) == cos(sin((sin(x) + cos(y)) * (sin(y) - cos(x)))) || \
     sin((x + sin(y)) * (sin(x) - y)) == cos(sin((sin(x) - cos(y)) * (sin(y) + cos(x)))) || \
     sin((x + sin(y)) * (sin(x) - y)) == cos(sin((sin(x) - cos(y)) * (sin(y) - cos(x)))) || \
     sin((x - sin(y)) * (sin(x) + y)) == cos(sin((sin(x) + cos(y)) * (sin(y) + cos(x)))) || \
     sin((x - sin(y)) * (sin(x) + y)) == cos(sin((sin(x) + cos(y)) * (sin(y) - cos(x)))) || \
     sin((x - sin(y)) * (sin(x) + y)) == cos(sin((sin(x) - cos(y)) * (sin(y) + cos(x)))) || \
     sin((x - sin(y)) * (sin(x) + y)) == cos(sin((sin(x) - cos(y)) * (sin(y) - cos(x)))) || \
     sin((x - sin(y)) * (sin(x) - y)) == cos(sin((sin(x) + cos(y)) * (sin(y) + cos(x)))) || \
     sin((x - sin(y)) * (sin(x) - y)) == cos(sin((sin(x) + cos(y)) * (sin(y) - cos(x)))) || \
     sin((x - sin(y)) * (sin(x) - y)) == cos(sin((sin(x) - cos(y)) * (sin(y) + cos(x)))) || \
     sin((x - sin(y)) * (sin(x) - y)) == cos(sin((sin(x) - cos(y)) * (sin(y) - cos(x))))" \
    -b 4.0 6.5 2.0 4.5
    ```

  - 📄 Highwire.gqs [🐌]

    ```
    "abs(x*cos(x) - y*sin(y)) == abs(x*cos(y) - y*sin(x))"
    ```

  - 📄 Trapezoidal Fortress.gqs [🐌]

    ```
    "abs(x*cos(x) + y*sin(y)) == x*cos(y) - y*sin(x)"
    ```

- 📂 Single Relation/Solid/

  - 📄 Sharp Threesome.gqs

    ```
    "(sin(sqrt((x + 5)^2 + y^2))) * (cos(8*atan(y/(x + 5)))) * \
     (sin(sqrt((x - 5)^2 + (y - 5)^2))) * (cos(8*atan((y - 5)/(x - 5)))) * \
     (sin(sqrt(x^2 + (y + 5)^2))) * (cos(8*atan((y + 5)/x))) > 0"
    ```

  - 📄 The Disco Hall.gqs

    ```
    "sin(abs(x + y)) > max(cos(x^2), sin(y^2))"
    ```

From http://www.peda.com/grafeq/reviews.html

- 1.

  ```
  "y == sqrt(x)^2"
  ```

- 2.

  ```
  "y == sqrt(x - 1)/sqrt(x - 3)"
  ```

- 3: the graph should be empty.

  ```
  "y == sqrt(x - 3)*sqrt(1 - x)"
  ```
