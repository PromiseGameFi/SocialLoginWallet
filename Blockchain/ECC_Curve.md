// Elliptic curve equation: y^2 = x^3 - 3x + 1
// Parametric form
f(t) = [
    t,
    sqrt(abs(t^3 - 3*t + 1)) * sign(t^3 - 3*t + 1),
    0
  ]
  
  // Create the positive part of the curve
  positive_curve = parametric(f(t), t, [-3, 3])
  
  // Create the negative part of the curve (mirror of positive part)
  negative_curve = parametric([f(t)[0], -f(t)[1], f(t)[2]], t, [-3, 3])
  
  // Add some height to make it 3D
  surface = parametricSurface([f(t)[0], f(t)[1], s*f(t)[1]], t, [-3, 3], s, [0, 0.5])
  
  // Set the view
  view = [-0.5, -3.5, 4.5]


Parametric Function
[t, sqrt(abs(t^3 - 3t + 1)) * sign(t^3 - 3t + 1), 0]
Set the Range to [-3, 3].
This creates the positive part of the curve. To create the negative part, add another Parametric Curve with the function:
[t, -sqrt(abs(t^3 - 3t + 1)) * sign(t^3 - 3t + 1), 0]
To add some 3D depth, click "+" again and select "Parametric Surface". Enter the function:
[t, ssqrt(abs(t^3 - 3t + 1)) * sign(t^3 - 3t + 1), ssqrt(abs(t^3 - 3t + 1)) * sign(t^3 - 3t + 1)]
For the Parametric Surface, set the Range for 't' to [-3, 3] and for 's' to [0, 0.5].
To adjust the view to match the 2D image more closely, click on the "⋮" menu in the top right and select "Edit Axes". Set the following:

Center: [0, 0, 0]
Scale: [1, 1, 0.5]
View: [-0.5, -3.5, 4.5]


For the positive part of the curve, use:
[t, sqrt(max(t^3 - 3*t + 1, 0)), 0]
For the negative part of the curve, use:
[t, -sqrt(max(t^3 - 3*t + 1, 0)), 0]
For the surface, use:
[t, s*sqrt(max(t^3 - 3*t + 1, 0)), s*sqrt(max(t^3 - 3*t + 1, 0))]


// Adjusted elliptic curve equation: y^2 = x^3 - 3x + 1
// Parametric form
f(t) = [
  t,
  sqrt(max(t^3 - 3*t + 1, 0)),
  0
]

// Create the full curve
curve = parametric([f(t)[0], f(t)[1], 0], t, [-2, 2])
curve2 = parametric([f(t)[0], -f(t)[1], 0], t, [-2, 2])

// Add a line to show intersection points
line = parametric([t, 0.5, 0], t, [-2, 2])

// Set the view
view = [0, 0, 5]