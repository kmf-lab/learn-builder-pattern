🦀 Learn Var‑Args in Rust

A hands‑on exploration of expressive, type‑safe flexibility

Most languages solve "variable arguments" with runtime tricks: overloaded functions, optional parameters, or loose argument lists.

Rust doesn’t — by design.

Instead, it gives us strong types, enums, builders, traits, and macros to express the same flexibility safely and explicitly.
This project walks through several idiomatic patterns that Rust programmers use to replace “var‑args” convenience with declarative, composable design.

---

🧭 What This Repository Demonstrates


Each module in src/ explores a different approach to handling configuration,

shape variation, or runtime diversity — without ever using varargs.


Module	Concept	Summary
- connection_enum.rs	Enum pattern	Models distinct data shapes (Tcp, Udp, LocalHost) explicitly — no invalid combinations.
- connection_builder_a.rs	Mutable (owned) builder	Classic fluent API; consumes self each step for single‑use construction.
- connection_builder_b.rs	Immutable (functional) builder	Reusable configuration pattern — pure, clone‑safe, ideal for concurrency and templating.
- connection_traits.rs	Traits: static & dynamic dispatch	Shows how T: Trait (static) and dyn Trait (dynamic) provide compile‑time and runtime polymorphism.
- connection_hybrid.rs	Enum + Trait (hybrid pattern)	Combines structured variant data with open‑ended behavior via trait objects.
- Macro + Builder	connect! macro	Syntactic sugar for ergonomic construction — compile‑time expansion, zero runtime cost.
- main.rs	Usage gallery	Demonstrates all patterns in action with concise examples.

---

🌱 Why It Matters


Rust’s design philosophy rejects ambiguity:

there’s no overload resolution, no “catch‑all” function.

Instead, you describe what can exist, and the compiler enforces how it must be built.

That forces stronger design decisions — but rewards you with:


- 🧩 Total exhaustiveness (match checks all variants)

- 🧱 Composability — build small, predictable types that combine cleanly

- 🧵 Thread‑safety by default (no shared mut varargs)

- ⚙️ Zero‑cost abstractions — pattern design, no runtime tax

- 💬 Readable intent — every variation is explicit in the type system

This project illustrates how to keep that safety without losing ergonomics or flexibility.


---

🧩 The Design Patterns at a Glance

Pattern	What It Models	Key Strengths	Use When
Enum	Data shape variations	Closed, type‑checked set of valid kinds	You know all possible variants ahead of time
Builder (owned)	Step‑by‑step object creation	Fluent, mutable, ergonomic	You build one object at a time
Builder (immutable)	Template & reuse	Thread‑safe, clone‑friendly	You need reusability or concurrent variation
Trait <T: Trait>	Compile‑time behavior abstraction	Zero runtime cost, full inlining	You need static polymorphism
Trait dyn Trait	Runtime polymorphism	One interface, many concrete types	Implementations vary at runtime (plugins, actions)
Hybrid (Enum + Trait)	Combined shape + behavior	Closed categories, open logic	You want structure and extensibility
Macro + Builder	Compact API syntax	Zero‑cost DSL over typed builders	You want var‑arg feel without runtime overhead

---

🚀 Try It

1. Clone the project

   git clone https://github.com/kmf-lab/learn_var_args_solutions.git
   cd learn_var_args_solutions

2. Build & run


Each demo lives in main.rs.

Uncomment one of the main() blocks to explore a particular pattern.


	cargo run

You’ll see printed examples showing how each pattern works and what it models.


---

🧠 Concept Summary


Rust doesn’t need var‑args.
It replaces them with structured variation —
each pattern is a form of controlled flexibility.


Level	Modeling Domain	You’re Learning To
Data	Enums & Builders	Shape evolving data safely
Behavior	Traits & Dispatch	Vary runtime logic cleanly
Hybrid	Enum + Trait	Merge data and behavior variation
Syntax	Macros	Add ergonomic, compile‑time sugar
These techniques recur throughout the ecosystem:


- Enums model state machines and protocol messages
- Builders configure clients, services, or tests
- Traits define interface boundaries across crates
- Macros bring all these together ergonomically


---

🧰 In Practice

If you’ve ever written overloaded constructors like:

	new Connection("10.0.0.1");
	new Connection("10.0.0.1", 443);
	new Connection("10.0.0.1", 443, true);

A possible Rust answer is:

	let a = connect!("10.0.0.1", 8080)?;
	let b = connect!("10.0.0.1", 443, true)?;

…and connect! expands to a strongly typed builder under the hood —
no reflection, no ambiguity, no runtime dispatch.  Check out the examples 
for many more approaches.
 

---

🪶 Philosophy


Rust’s type system makes design intentions part of the code.

What other languages hide behind var‑args, Rust makes explicit — and therefore safe.


Flexibility isn’t lost; it’s just described precisely.


