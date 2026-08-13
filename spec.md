# Features
1. The product is a general-purpose presentation editor for individuals, teams,
   designers, educators, students, and business users.
2. The beta product should atleast support user auth and persistence
3. The same editor experience ships as a browser application and as desktop
   applications for Windows, macOS, and Linux.
4. `.pptx` is the only presentation file format in scope. Users may insert
   supported image, audio, and video assets, but legacy `.ppt`, PDF, Keynote,
   and Google Slides conversion are out of scope.
5. The initial Beta creates, edits, organizes, imports, and exports complete
   decks. Slideshow playback, presenter mode, authoring animations, and
   authoring transitions are out of scope.
6. Existing unsupported PPTX package content must be retained whenever safe; it
   must never be discarded silently.
7. Common PowerPoint objects—including charts and media—must be editable rather
   than flattened or treated only as opaque objects.

# Tech Stack

- UI: React with TypeScript
- Desktop Shell: electron
- Document Engine: Rust compiled to WebAssembly
- Renderer: WebGL
- pptx processing:OOXML for processing pptx into structured data
- Browser persistence: postgres

# Scope
