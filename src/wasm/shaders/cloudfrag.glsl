// Raymarched volumetric cloud: noise-based density inside an SDF “puff” shape.
// Bright, soft, realistic-white cloud with gentle sky ambient.
// Mouse interactivity (dent + subtle sheen) is controlled by u_mouse_mix (hero=1, bg=0).

precision highp float;

varying vec2 v_uv;

uniform float u_time;
uniform vec2  u_resolution;
uniform vec3  u_sun_dir;
uniform vec2  u_mouse;
uniform float u_seed;

// 0.0 = ignore mouse interactivity (background clouds)
// 1.0 = enable mouse interactivity (hero cloud)
uniform float u_mouse_mix;

// ---- tweak knobs ----
const float STEP = 0.035;
const float TMAX = 5.0;

// mouse blob size (bigger = larger interaction area)
const float MOUSE_RADIUS = 6.0;

// dent depth in "cloud space" (bigger = deeper dent)
const float DENT_DEPTH = 0.65;

// keeps the dent mostly on the front surface (prevents tunnel look)
const float FRONT_ONLY_MIN = 0.65; // higher = more front-only
const float FRONT_ONLY_MAX = 0.98;

// subtle sheen so you can still “see” interaction on a white cloud
const float SHEEN_STRENGTH = 0.18;

float hash(float n) { return fract(sin(n) * 43758.5453); }
float rand(float n) { return fract(sin(n) * 43758.5453); }

float noise(vec3 x) {
  vec3 p = floor(x);
  vec3 f = fract(x);
  f = f * f * (3.0 - 2.0 * f);

  float n = p.x + p.y * 57.0 + 113.0 * p.z;
  return mix(
    mix(mix(hash(n + 0.0),  hash(n + 1.0),  f.x),
        mix(hash(n + 57.0), hash(n + 58.0), f.x), f.y),
    mix(mix(hash(n + 113.0), hash(n + 114.0), f.x),
        mix(hash(n + 170.0), hash(n + 171.0), f.x), f.y),
    f.z
  );
}

float fbm(vec3 p) {
  float s = 0.0;
  float a = 0.55;
  for (int i = 0; i < 5; i++) {
    s += a * noise(p);
    p *= 2.02;
    a *= 0.5;
  }
  return s;
}

float sdSphere(vec3 p, vec3 c, float r) { return length(p - c) - r; }

float smin(float a, float b, float k) {
  float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
  return mix(b, a, h) - k * h * (1.0 - h);
}

float sdCloudShape(vec3 p) {
  p.x *= 1.05;
  float k = 0.12;

  float r1 = rand(u_seed * 12.9898);
  float r2 = rand(u_seed * 78.233);
  float r3 = rand(u_seed * 39.425);

  float d1 = sdSphere(p, vec3(-0.45 + r1 * 0.15, 0.00, 0.0), 0.55);
  float d2 = sdSphere(p, vec3( 0.35 - r2 * 0.15, 0.05, 0.0), 0.65);
  float d3 = sdSphere(p, vec3( 0.00, 0.30 + r3 * 0.10, 0.0), 0.50);

  vec2 jitter = (vec2(r1, r2) - 0.5) * 0.35;
  p.xy += jitter;

  p.x *= mix(0.95, 1.1, r3);

  float d = smin(d1, d2, k);
  d = smin(d, d3, k);
  d = max(d, -(p.y + 0.45));
  return d;
}

float densityAt(vec3 p, float time) {
  float d = sdCloudShape(p);
  if (d > 0.2) return 0.0;

  float r2 = rand(u_seed * 78.233);

  float base = smoothstep(0.25, -0.25, d);
  vec3 np = p * (2.1 + r2 * 0.6) + vec3(0.0, time * 0.12, 0.0);

  float n = fbm(np);

  float puff = smoothstep(0.35, 0.9, n);
  puff = mix(0.35, 1.0, puff);

  return base * puff;
}

vec3 densityNormal(vec3 p, float time) {
  float e = 0.02;
  float dx = densityAt(p + vec3(e, 0, 0), time) - densityAt(p - vec3(e, 0, 0), time);
  float dy = densityAt(p + vec3(0, e, 0), time) - densityAt(p - vec3(0, e, 0), time);
  float dz = densityAt(p + vec3(0, 0, e), time) - densityAt(p - vec3(0, 0, e), time);
  return normalize(vec3(dx, dy, dz) + 1e-6);
}

void main() {
  vec2 uv = v_uv * 2.0 - 1.0;
  uv.x *= u_resolution.x / u_resolution.y;

  // mouse in same uv space (screen-space mask)
  vec2 muv = (u_mouse / u_resolution) * 2.0 - 1.0;
  muv.y = -muv.y;
  muv.x *= u_resolution.x / u_resolution.y;

  vec2 dUV = uv - muv;
  float mouseMask = exp(-dot(dUV, dUV) * MOUSE_RADIUS) * u_mouse_mix;

  // camera
  vec3 ro = vec3(0.0, 0.0, 2.7);
  vec3 rd = normalize(vec3(uv * 0.85, -1.8));

  float time = u_time;
  vec3 cloudOffset = vec3(0.0, 0.08 * sin(u_time * 0.8), 0.0);

  vec3 sunDir = normalize(u_sun_dir);

  // dither to reduce banding
  float t = 0.0;
  float dither = hash(dot(gl_FragCoord.xy, vec2(12.9898, 78.233)) + u_seed * 13.7);
  t += (dither - 0.5) * STEP * 0.9;

  vec3 sum = vec3(0.0);
  float trans = 1.0;

  for (int i = 0; i < 72; i++) {
    if (t > TMAX || trans < 0.02) break;

    vec3 p = ro + rd * t - cloudOffset;

    // base density (to know if we’re inside cloud)
    float dens0 = densityAt(p, time);

    if (dens0 > 0.001) {
      // only apply interaction where cloud exists
      float hit = smoothstep(0.02, 0.18, dens0);

      // IMPORTANT: apply the dent mostly near the front surface.
      // trans starts at 1 and decreases as we accumulate.
      float front = smoothstep(FRONT_ONLY_MIN, FRONT_ONLY_MAX, trans);

      float influence = mouseMask * hit * front;

      // DENT: push sampling deeper along the viewing ray.
      // This “moves the surface inward” instead of removing material.
      vec3 p_def = p + rd * (influence * DENT_DEPTH);

      float dens = densityAt(p_def, time);
      vec3 n = -densityNormal(p_def, time);

      // lighting (your current nice white look)
      vec3 albedo = vec3(0.98, 0.99, 1.00);
      vec3 sunCol = vec3(1.00, 0.98, 0.95);
      vec3 skyCol = vec3(0.55, 0.68, 0.92);

      float ndl = clamp(dot(n, sunDir), 0.0, 1.0);
      float diff = pow(ndl, 0.85);
      float wrapped = diff * 0.75 + 0.25;

      float shadow = 1.0;
      float lt = 0.04;
      for (int s = 0; s < 8; s++) {
        float ld = densityAt(p_def + sunDir * lt, time);
        shadow *= exp(-ld * 1.35);
        lt += 0.075;
      }
      shadow = clamp(shadow, 0.35, 1.0);

      float rim = pow(1.0 - clamp(dot(n, -rd), 0.0, 1.0), 2.0);

      vec3 light = skyCol * 0.75;
      light += sunCol * (wrapped * shadow) * 0.95;
      light += rim * vec3(0.18, 0.20, 0.24);

      vec3 lit = albedo * light;

      // subtle “sheen” so interaction is visible even when cloud is bright
      lit += influence * SHEEN_STRENGTH * vec3(0.25, 0.35, 0.55);

      // opacity from density (NO alpha killing -> avoids tunnel/hole)
      float alpha = 1.0 - exp(-dens * 2.8);

      sum += trans * alpha * lit;
      trans *= (1.0 - alpha);
    }

    t += STEP;
  }

  float outA = 1.0 - trans;
  vec3 outCol = (outA > 1e-4) ? (sum / outA) : vec3(0.0);

  outCol = pow(clamp(outCol, 0.0, 1.0), vec3(1.0 / 2.2));
  gl_FragColor = vec4(outCol, outA);
}
