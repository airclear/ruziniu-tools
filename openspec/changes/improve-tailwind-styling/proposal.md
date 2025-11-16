# Improve Tailwind Styling for URL Converter

## Why
The existing batch short URL to long URL converter functionality is already implemented, but the user interface currently uses custom CSS instead of Tailwind CSS. To improve maintainability, consistency, and development velocity, we need to migrate the existing styling to use Tailwind CSS classes.

## What Changes
- Replace custom CSS styles with Tailwind CSS utility classes
- Maintain the same visual appearance and functionality
- Ensure responsive design behavior
- Add Tailwind CSS as a dependency to the project

## Impact
- Affected specs: `short2long-url` (modify existing styling requirements)
- Affected code: `src/App.vue` (template and styles), `package.json` (dependencies)
- New dependencies: Tailwind CSS and its configuration