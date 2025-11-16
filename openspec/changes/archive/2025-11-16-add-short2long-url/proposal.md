# Change: Add batch short2long-url conversion feature

## Why
The application needs a feature to convert a list of shortened URLs to their original, longer URLs in batch. This is useful for security and for understanding where a link will lead before clicking on it, especially when dealing with many URLs at once.

## What Changes
- A new UI component will be added to the frontend to:
  - Accept a list of short URLs in a text area.
  - Allow users to set the concurrency level.
  - Display the progress of the conversion.
  - Allow users to download the results as a CSV file.
- A new backend command `batch_short2long_url` will be created in the Rust application to handle the concurrent URL expansion.

## Impact
- Affected specs: `short2long-url` (new)
- Affected code: `src/App.vue`, `src-tauri/src/main.rs`
