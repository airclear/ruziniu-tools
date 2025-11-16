## 1. Backend
- [ ] 1.1 Create the `batch_short2long_url` command in the Rust backend that accepts a list of URLs and a concurrency setting.
- [ ] 1.2 Implement concurrent URL expansion in the backend.
- [ ] 1.3 Implement progress reporting from the backend to the frontend.
- [ ] 1.4 Write tests for the backend command.

## 2. Frontend
- [ ] 2.1 Create a new UI component for batch URL conversion.
- [ ] 2.2 Add a textarea for pasting a list of short URLs.
- [ ] 2.3 Add an input for setting the concurrency level.
- [ ] 2.4 Add a "Start" button to initiate the conversion.
- [ ] 2.5 Add a progress bar to display the conversion progress.
- [ ] 2.6 Add a "Download" button that becomes active when the conversion is complete.
- [ ] 2.7 Implement the logic to call the `batch_short2long_url` backend command.
- [ ] 2.8 Implement the logic to generate and download the CSV file with "短链" and "长链" headers.
- [ ] 2.9 Write tests for the frontend component.
