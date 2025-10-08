# Changelog

## 0.3.0 (2025-10-08)
- Bump gradle wrapper version to `9.1`, supporting Java 25 

## 0.2.3 (2024-10-04)
- Add `set_fixed_layout` on `Table`

## 0.2.2 (2024-02-20)
- Add `get_width()` and `get_height()` on `ImageData`
- Add `get_page` and `get_number_of_pages` on `PdfDocument`
- Add `PdfCanvas`
- Add `PdfPage`
- Add `Rectangle`
- Add `PdfExtGState`
- Add `RootElement` trait
- Add `new_with_flush` to `Document`
- Add `Canvas`

## 0.2.1 (2024-01-08)
- Add `set_margin_left()`, `set_margin_right()` on the BlockElement trait
- Add `Cell::new_with_span`

## 0.2.0 (2023-06-19)
- Reworked API to be more Trait based
- Reworked API to return `&Self`
- Add Font support
- Add various missing, but essential, functions in e.g. `ElementPropertyContainer`

## 0.1.0 (2023-06-15)
- Initial version
