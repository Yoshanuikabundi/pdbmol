pub fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn norm(v: [f32; 3]) -> f32 {
    dot(v, v).sqrt()
}

pub fn unit(v: [f32; 3]) -> [f32; 3] {
    [v[0] / norm(v), v[1] / norm(v), v[2] / norm(v)]
}

pub fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn mul(s: f32, v: [f32; 3]) -> [f32; 3] {
    [s * v[0], s * v[1], s * v[2]]
}
