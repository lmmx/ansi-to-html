import pytest
import ansi_to_html


def test_default_converter():
    """Test the default Converter configuration"""
    converter = ansi_to_html.Converter()

    # Simple ANSI text with colors
    bold = "\033[1m"
    red = "\033[31m"
    reset = "\033[0m"
    input_text = f"Hello {bold}{red}World{reset}!"

    html = converter.convert(input_text)

    # The default conversion should escape HTML characters and use color styles
    assert "<b>" in html
    assert "color:var(--red,#a00)" in html
    assert "World" in html


def test_skip_escape():
    """Test the skip_escape flag"""
    converter = ansi_to_html.Converter(skip_escape=True)

    input_text = "<div>\033[31mRed Text\033[0m</div>"
    html = converter.convert(input_text)

    # The HTML tags should not be escaped
    assert "<div>" in html
    assert "</div>" in html

    # But default prefix should still be used
    assert "color:var(--red,#a00)" in html


def test_skip_optimize():
    """Test the skip_optimize flag"""
    converter = ansi_to_html.Converter(skip_optimize=True)

    # Create some ANSI text that would normally be optimized
    input_text = "\033[31m\033[32mGreen Text\033[0m"
    html = converter.convert(input_text)

    # In non-optimized mode, we might see multiple spans (implementation-dependent)
    assert "Green Text" in html


def test_four_bit_var_prefix():
    """Test the four_bit_var_prefix flag"""
    converter = ansi_to_html.Converter(four_bit_var_prefix="custom-")

    input_text = "\033[31mRed Text\033[0m"
    html = converter.convert(input_text)

    # Should use the custom prefix for CSS variables
    assert "color:var(--custom-red,#a00)" in html


def test_multiple_flags():
    """Test multiple flags combined"""
    converter = ansi_to_html.Converter(
        skip_escape=True, skip_optimize=True, four_bit_var_prefix="theme-"
    )

    input_text = "<div>\033[31mRed Text\033[0m</div>"
    html = converter.convert(input_text)

    # All settings should be applied
    assert "<div>" in html  # HTML not escaped
    assert "</div>" in html  # HTML not escaped
    assert "color:var(--theme-red,#a00)" in html  # Custom prefix applied


def test_direct_convert_function():
    """Test that the top-level convert function still works"""
    bold = "\033[1m"
    red = "\033[31m"
    reset = "\033[0m"
    input_text = f"Hello {bold}{red}World{reset}!"

    html = ansi_to_html.convert(input_text)

    # Should perform default conversion
    assert "<b>" in html
    assert "World" in html


@pytest.mark.parametrize(
    "escape_html,optimize,prefix,input_text,expected",
    [
        # Test case 1: Default settings
        (
            False,
            False,
            None,
            "<div>\033[31mTest\033[0m</div>",
            ["&lt;div&gt;", "color:var(--red,#a00)", "&lt;/div&gt;"],
        ),
        # Test case 2: Skip escape
        (
            True,
            False,
            None,
            "<div>\033[31mTest\033[0m</div>",
            ["<div>", "color:var(--red,#a00)", "</div>"],
        ),
        # Test case 3: Custom prefix
        (
            False,
            False,
            "my-",
            "\033[31mColored\033[0m",
            ["color:var(--my-red,#a00)", "Colored"],
        ),
        # Test case 4: Skip escape and custom prefix
        (
            True,
            True,
            "fancy-",
            "<b>\033[31mBold Red\033[0m</b>",
            ["<b>", "color:var(--fancy-red,#a00)", "Bold Red", "</b>"],
        ),
    ],
)
def test_converter_parametrized(escape_html, optimize, prefix, input_text, expected):
    """Parametrized test for different converter configurations"""
    converter = ansi_to_html.Converter(
        skip_escape=escape_html, skip_optimize=optimize, four_bit_var_prefix=prefix
    )

    html = converter.convert(input_text)

    # Check all expected substrings are in the output
    for substr in expected:
        assert substr in html
