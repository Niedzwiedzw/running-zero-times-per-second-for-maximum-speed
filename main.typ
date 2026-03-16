#set page(
  width: 16cm,
  height: 9cm,
  margin: 1cm,
  fill: black,
  background: place(
    top + right,
    dx: -0.5cm,
    dy: 0.5cm,
    image("logo.svg", width: 1.5cm)
  )
)

#set text(
  fill: white,
  font: "Iosevka",
  size: 14pt
)

#show heading: set text(fill: white)

// Slide 1: Introduction
#align(center + horizon)[
  #text(size: 24pt, weight: "bold")[
    Running code 0 times / second for maximum performance
  ]

  #v(2cm)

  #text(size: 18pt)[Wojciech Brożek]

  #v(0.5cm)

  #link("https://github.com/Niedzwiedzw")[github.com/Niedzwiedzw] \
  #link("mailto:wojciech.brozek@niedzwiedz.it")[wojciech.brozek\@niedzwiedz.it] \
  #link("https://exein.io")[exein.io]

  ```rust
  trait Exein: Rust + Security {}
  ```
]
