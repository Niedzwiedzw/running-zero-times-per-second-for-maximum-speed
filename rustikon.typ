#let rustikon(body) = {
  set page(
    width: 16cm,
    height: 9cm,
    margin: 1cm,
    fill: black,
    background: {
      place(
        top + left,
        dx: 0.6cm,
        dy: 0.5cm,
        image("rustikon-logo.png", width: 3.4cm),
      )
      place(
        top + right,
        dx: -0.5cm,
        dy: 0.5cm,
        image("logo.svg", width: 2.5cm),
      )
    },
  )

  set text(
    fill: white,
    font: "Iosevka",
    size: 14pt,
  )

  show heading: set text(fill: white)

  body
}
