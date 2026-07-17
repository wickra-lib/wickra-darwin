"""The Python surface exposes exactly the documented API."""

import wickra_darwin
from wickra_darwin import Darwin


def test_module_exports() -> None:
    assert set(wickra_darwin.__all__) == {"Darwin", "__version__"}


def test_darwin_methods() -> None:
    for name in ("command", "version"):
        assert hasattr(Darwin, name)


def test_version_is_a_string() -> None:
    assert isinstance(wickra_darwin.__version__, str)
    assert wickra_darwin.__version__
