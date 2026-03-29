use leptos::*;

#[derive(Clone)]
struct Project {
    title: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
    link: Option<&'static str>,
    year: &'static str,
}

#[derive(Clone)]
struct Achievement {
    title: &'static str,
    context: &'static str,
    year: &'static str,
    kind: &'static str,
}

fn projects() -> Vec<Project> {
    vec![
        Project {
            title: "Lua++ Compiler",
            description: "A compiler for my very own programming language, Lua++.",
            tags: &["Compilers", "Lua", "Languages"],
            link: Some("https://github.com/yeicebear/Luapp"),
            year: "2026",
        },
        Project {
            title: "Rake",
            description: "Worked with a server member, Noer, to create a make-like build system. It's designed to be minimal, easy to use, read and implement.",
            tags: &["Rust", "Build Systems", "CLI"],
            link: Some("https://github.com/yeicebear/Rake"),
            year: "2026",
        },
        Project {
            title: "MineWeather",
            description: "Compares your local weather data to a Minecraft biome scene using Three.js and the openWeather API. It all runs in the browser.",
            tags: &["Three.js", "Weather API", "WebGL", "Creative"],
            link: None,
            year: "2026",
        },
        Project {
            title: "rocket-runtime (contributor)",
            description: "Open-source contribution to xNoerPlaysCodes/rocket-runtime. Added Rust interop and hardened memory safety in several high-risk sections of the runtime.",
            tags: &["Rust", "Open Source", "Memory Safety", "Runtimes"],
            link: Some("https://github.com/xNoerPlaysCodes/rocket-runtime"),
            year: "2026",
        },
        Project {
            title: "This Portfolio",
            description: "Built in Rust using the Leptos framework, compiled to WebAssembly. Zero JavaScript runtime. I built this to organize my creations and contributions.",
            tags: &["Rust", "Leptos", "WASM"],
            link: Some("https://yeicebear.github.io"),
            year: "2026",
        },
    ]
}

fn achievements() -> Vec<Achievement> {
    vec![
        Achievement {
            title: "Compoudon 2024:- Gold Medal by merit, Top 100",
            context: "National olympiad-style competition. Ranked in the top 100 by merit.",
            year: "7th grade",
            kind: "award",
        },
        Achievement {
            title: "ThinkQuest 2024:- School Winner",
            context: "In the team of 3 that won the inter-school level competition.",
            year: "7th grade",
            kind: "award",
        },
        Achievement {
            title: "Silverzone IT Olympiad 2025:- Gold Medal",
            context: "Gold medal, class topper in the Information Technology category.",
            year: "8th grade",
            kind: "award",
        },
        Achievement {
            title: "Silverzone Olympiad of Science",
            context: "Gold medal, class topper in the national Silverzone science olympiad.",
            year: "4th grade",
            kind: "award",
        },
        Achievement {
            title: "GMDL",
            context: "Participated in GMDL, (Global Maths Debate League).",
            year: "8th grade",
            kind: "award",
        },
        Achievement {
            title: "PacMUN 2025:- Verbal Mention, UNEP",
            context: "Received a verbal mention at Pacific Model United Nations 2025.",
            year: "2025",
            kind: "award",
        },
        Achievement {
            title: "Dev Community — Admin, Co-Owner and Founder",
            context: "Run a ~80-member developer community. Members regularly collaborate on projects like browsers and desktop environments.",
            year: "Ongoing",
            kind: "community",
        },
    ]
}
#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <nav class="nav">
                <span class="nav-name">"[Aadyansh Rai]"</span>
                <div class="nav-links">
                    <a href="#about">"About"</a>
                    <a href="#projects">"Projects"</a>
                    <a href="#achievements">"Achievements"</a>
                    <a href="#contact">"Contact"</a>
                </div>
            </nav>
        </header>
    }
}
#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero" id="about">
            <div class="hero-kicker">"Builder. Thinker. Perpetually curious."</div>
            <h1 class="hero-title">
        I <em>" make things"</em>;<br/>
        I <em>" break things"</em>.
            </h1>
            <div class="hero-body">
                <p>
                    "I'm a student, as of 29/3/26 in High School. I enjoy building or contributing to projects like compilers, graphics libraries or really, anything code!!"
                </p>
                <p>
                    "Outside of code I make & enjoy analog horror and SFM films, play basketball, "
                    "and love Subnautica lore, as well as the gameplay (of course)!"
                </p>
            </div>
            <div class="hero-links">
                <a href="https://github.com/yeicebear/" class="btn" target="_blank">"GitHub"</a>
                <a href="mailto:icebearisreal1@gmail.com" class="btn btn-outline">"Email"</a>
            </div>
        </section>
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <article class="project-card">
            <div class="project-top">
                <span class="project-year">{project.year}</span>
                {project.link.map(|url| view! {
                    <a href={url} class="project-ext" target="_blank">"↗"</a>
                })}
            </div>
            <h3 class="project-title">{project.title}</h3>
            <p class="project-desc">{project.description}</p>
            <div class="project-tags">
                {project.tags.iter().map(|t| view! {
                    <span class="tag">{*t}</span>
                }).collect_view()}
            </div>
        </article>
    }
}
#[component]
fn Projects() -> impl IntoView {
    let items = projects();
    view! {
        <section class="section" id="projects">
            <div class="section-header">
                <h2 class="section-title">"Projects"</h2>
                <p class="section-sub">"Things I've built."</p>
            </div>
            <div class="project-grid">
                {items.into_iter().map(|p| view! { <ProjectCard project=p /> }).collect_view()}
            </div>
        </section>
    }
}
#[component]
fn AchievementRow(item: Achievement) -> impl IntoView {
    view! {
        <div class="ach-row">
            <div class="ach-left">
                <span class="ach-year">{item.year}</span>
                <span class={format!("ach-badge ach-{}", item.kind)}>{item.kind}</span>
            </div>
            <div class="ach-right">
                <p class="ach-title">{item.title}</p>
                <p class="ach-context">{item.context}</p>
            </div>
        </div>
    }
}

#[component]
fn Achievements() -> impl IntoView {
    let items = achievements();
    view! {
        <section class="section section-alt" id="achievements">
            <div class="section-header">
                <h2 class="section-title">"Achievements"</h2>
                <p class="section-sub">"Competitions, contributions, community."</p>
            </div>
            <div class="ach-list">
                {items.into_iter().map(|a| view! { <AchievementRow item=a /> }).collect_view()}
            </div>
        </section>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <section class="section" id="contact">
            <div class="contact-inner">
                <h2 class="contact-title">"Get in touch."</h2>
                <p class="contact-sub">
                    "I love finding new projects and things to add here!"
                </p>
                <a href="mailto:icebearisreal1@gmail.com" class="contact-link">"icebearisreal1@gmail.com"</a>
            </div>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <p>"Built in Rust + Leptos + WASM. Luckily didn't have to touch any JS lol."</p>
        </footer>
    }
}
#[component]
fn App() -> impl IntoView {
    view! {
        <Header />
        <main>
            <Hero />
            <Projects />
            <Achievements />
            <Contact />
        </main>
        <Footer />
    }
}

fn main() {
    mount_to_body(App)
}