// Mobile-first: cheap. Desktop (sm+): your original heavy glass.

pub const GLASS_HERO: &str = "
relative mx-auto
rounded-[32px]
backdrop-blur-none sm:backdrop-blur-2xl

bg-gradient-to-br
from-white/18 via-white/10 to-white/5
sm:from-white/25 sm:via-white/12 sm:to-white/5

border border-white/25 sm:border-white/30
shadow-[0_14px_36px_-18px_rgba(0,0,0,0.45)] sm:shadow-[0_30px_80px_-20px_rgba(0,0,0,0.55)]

before:absolute before:inset-0 before:rounded-[32px] before:pointer-events-none
before:bg-gradient-to-b
before:from-white/35 before:via-transparent before:to-transparent
before:opacity-25 sm:before:opacity-50

after:absolute after:inset-0 after:rounded-[32px] after:pointer-events-none
after:shadow-[inset_0_1px_1px_rgba(255,255,255,0.35)] sm:after:shadow-[inset_0_1px_1px_rgba(255,255,255,0.5)]

sm:transition-transform sm:duration-700 sm:ease-out
sm:hover:scale-[1.01]
";

pub const GLASS_NAV: &str = "
relative
backdrop-blur-none sm:backdrop-blur-xl
bg-white/10 sm:bg-gradient-to-b sm:from-white/20 sm:via-white/10 sm:to-white/5
border border-white/20
rounded-full
shadow-none sm:shadow-[0_10px_30px_-10px_rgba(0,0,0,0.35)]
text-white/80
before:absolute before:inset-0 before:rounded-full before:pointer-events-none
before:bg-gradient-to-b before:from-white/35 before:via-transparent before:to-transparent before:opacity-25
after:absolute after:inset-0 after:rounded-full after:pointer-events-none
after:shadow-[inset_0_1px_1px_rgba(255,255,255,0.30)]
";

pub const NAV_LINK: &str = "
px-2.5 sm:px-3
py-1.5
text-sm font-medium
text-white/70
whitespace-nowrap
transition duration-300
hover:text-white
";

pub const GLASS_TERMINAL: &str = "
relative overflow-hidden rounded-3xl
border border-white/10
bg-gradient-to-br from-slate-950/60 via-slate-950/35 to-slate-900/25
backdrop-blur-none sm:backdrop-blur-xl
shadow-[0_16px_44px_-22px_rgba(0,0,0,0.70)] sm:shadow-[0_30px_90px_-40px_rgba(0,0,0,0.80)]
before:absolute before:inset-0 before:pointer-events-none before:rounded-3xl
before:bg-gradient-to-b before:from-white/16 before:via-transparent before:to-transparent
before:opacity-25 sm:before:opacity-40
after:absolute after:inset-0 after:pointer-events-none after:rounded-3xl
after:shadow-[inset_0_1px_1px_rgba(255,255,255,0.08)] sm:after:shadow-[inset_0_1px_1px_rgba(255,255,255,0.10)]
";

pub const GLASS_SKILLS: &str = "
relative overflow-hidden rounded-3xl
border border-white/15
bg-gradient-to-br from-white/12 via-white/7 to-white/4
sm:from-white/14 sm:via-white/8 sm:to-white/4
backdrop-blur-none sm:backdrop-blur-xl
shadow-[0_14px_38px_-20px_rgba(0,0,0,0.45)] sm:shadow-[0_20px_55px_-25px_rgba(0,0,0,0.55)]
px-6 py-6
before:absolute before:inset-0 before:pointer-events-none before:rounded-3xl
before:bg-gradient-to-b before:from-white/22 before:via-transparent before:to-transparent
before:opacity-18 sm:before:opacity-25
after:absolute after:inset-0 after:pointer-events-none after:rounded-3xl
after:shadow-[inset_0_1px_1px_rgba(255,255,255,0.12)] sm:after:shadow-[inset_0_1px_1px_rgba(255,255,255,0.18)]
sm:transition sm:duration-300
sm:hover:border-white/25 sm:hover:bg-white/10
";

pub const GLASS_SKILL_ENTRY: &str = "
inline-flex items-center gap-2
rounded-full
border border-white/10
bg-white/5
px-3 py-1
text-xs
text-white/75
backdrop-blur-none sm:backdrop-blur
sm:transition sm:duration-300
sm:hover:bg-white/10 sm:hover:text-white/90
";

pub const GLASS_CARD: &str = "
relative overflow-hidden rounded-3xl
backdrop-blur-none sm:backdrop-blur-xl
bg-gradient-to-br from-white/16 via-white/9 to-white/5
sm:from-white/20 sm:via-white/10 sm:to-white/5
border border-white/20
shadow-[0_14px_40px_-18px_rgba(0,0,0,0.40)] sm:shadow-[0_20px_50px_-15px_rgba(0,0,0,0.45)]
text-white/90
before:absolute before:inset-0 before:rounded-3xl before:pointer-events-none
before:bg-gradient-to-b before:from-white/26 before:via-transparent before:to-transparent
before:opacity-22 sm:before:opacity-35
after:absolute after:inset-0 after:rounded-3xl after:pointer-events-none
after:shadow-[inset_0_1px_1px_rgba(255,255,255,0.18)] sm:after:shadow-[inset_0_1px_1px_rgba(255,255,255,0.25)]
";

pub const BTN_PRIMARY: &str = "
inline-flex items-center justify-center gap-2
rounded-xl px-4 py-2 text-sm font-medium
bg-white/10 text-white/85
border border-white/15
backdrop-blur-none sm:backdrop-blur
sm:transition sm:duration-300
sm:hover:bg-white/15 sm:hover:border-white/25 sm:hover:text-white
";

pub const BTN_GHOST: &str = "
inline-flex items-center justify-center gap-2
rounded-xl px-4 py-2 text-sm font-medium
text-white/70
border border-white/10
bg-white/5
backdrop-blur-none sm:backdrop-blur
sm:transition sm:duration-300
sm:hover:bg-white/10 sm:hover:border-white/20 sm:hover:text-white
";

pub const SOCIAL_TILE: &str = "
group relative overflow-hidden rounded-2xl
border border-white/12 bg-white/5
backdrop-blur-none sm:backdrop-blur
p-4
sm:transition sm:duration-300
sm:hover:bg-white/8 sm:hover:border-white/22
";
