#define_import_path cel::utils

// apache license: https://gitlab.com/s-ilent/filamented/-/blob/master/Filamented/SharedFilteringLib.hlsl

fn cubic(v: f32) -> vec4<f32> {
    let n = vec4<f32>(1.0, 2.0, 3.0, 4.0) - v;
    let s = n * n * n;
    let x = s.x;
    let y = s.y - 4.0 * s.x;
    let z = s.z - 4.0 * s.y + 6.0 * s.x;
    let w = 6.0 - x - y - z;
    return vec4<f32>(x, y, z, w);
}

fn textureBicubic(tex: texture_2d<f32>, smp: sampler, coord: vec2<f32>) -> vec4<f32>{

   let texSize = vec2<f32>(textureDimensions(tex));
   let invTexSize = vec2<f32>(1.0) / texSize;
   
   let coord = coord * texSize - 0.5;

   
    let fxy = fract(coord);
    let coord = coord - fxy;

    let xcubic = cubic(fxy.x);
    let ycubic = cubic(fxy.y);

    let c = coord.xxyy + vec2<f32>(-0.5, 1.5).xyxy;
    
    let s = vec4(xcubic.xz + xcubic.yw, ycubic.xz + ycubic.yw);
    let offset = c + vec4 (xcubic.yw, ycubic.yw) / s;
    
    let offset = offset * invTexSize.xxyy;
    
    let sample0 = textureSample(tex, smp, offset.xz);
    let sample1 = textureSample(tex, smp, offset.yz);
    let sample2 = textureSample(tex, smp, offset.xw);
    let sample3 = textureSample(tex, smp, offset.yw);

    let sx = s.x / (s.x + s.y);
    let sy = s.z / (s.z + s.w);

    return mix(
       mix(sample3, sample2, sx), mix(sample1, sample0, sx)
    , sy);
}
