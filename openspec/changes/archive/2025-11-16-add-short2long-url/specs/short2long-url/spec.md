## ADDED Requirements
### Requirement: Batch convert short URLs to long URLs
The system SHALL provide a way to convert a list of short URLs to their corresponding long URLs.

#### Scenario: Successful batch conversion
- **GIVEN** a user has entered a list of valid short URLs in the text area
- **AND** the user has set a concurrency level
- **WHEN** the user clicks the "Start" button
- **THEN** the system SHALL start the conversion process
- **AND** the system SHALL display the overall progress of the conversion
- **AND** upon completion, the system SHALL enable a "Download" button.

#### Scenario: Download results
- **GIVEN** the batch conversion is complete
- **WHEN** the user clicks the "Download" button
- **THEN** the system SHALL generate a CSV file containing the results
- **AND** the CSV file SHALL have two columns with headers "短链" and "长链"
- **AND** the browser SHALL prompt the user to save the CSV file.

#### Scenario: Handling invalid URLs in batch
- **GIVEN** a user has entered a list of URLs containing some invalid or non-existent short URLs
- **WHEN** the conversion process is running
- **THEN** the system SHALL record an error for each invalid URL
- **AND** the final CSV file SHALL contain the short URL and an error message in the "长链" column for the failed URLs.
