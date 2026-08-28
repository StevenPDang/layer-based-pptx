1. JSON round-trip test
    - Add serde_json as a dev dependency.
    - Create one presentation with one slide and one positioned text layer.
    - Serialize, deserialize, and assert equality.
    - Assert important JSON names such as slideId and layerId.

2. Empty ID validation
    - Reject empty or whitespace-only SlideId.
    - Reject empty or whitespace-only LayerId.
    - Report the correct slide/layer index.
    - Add a test for each rule.

3. Layer geometry validation
    - Reject layer width and height values less than or equal to zero.
    - Allow negative x and y, unless you decide otherwise.
    - Add width and height tests.

4. Duplicate ID validation
    - Detect duplicate slide IDs.
    - Detect duplicate layer IDs.
    - Decide whether layer IDs must be unique within each slide or throughout the presentation.
    - Add tests for both duplicate rules.

5. Successful validation test
    - Construct a fully valid presentation.
    - Assert validate(&presentation) == Ok(()).

6. Multiple-error accumulation test
    - Construct a document with several problems.
    - Verify validation returns every error rather than stopping at the first.

7. Geometry and ID tests
    - Confirm Emu values serialize as intended.
    - Confirm SlideId and LayerId serialize as strings.
    - Confirm negative positions round-trip correctly.

8. Public API documentation
    - Add /// documentation to every public type and public function.
    - Explain that Emu means English Metric Units.
    - Document the validity rules and error-location semantics.

9. Record the geometry decision
    - Update the living spec to state that EMUs are the canonical internal unit.
    - The current code has made that decision, but the spec still lists it as open.