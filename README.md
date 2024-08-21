# About
A reimplementation of a few CoreGraphics types in Rust. Rather than generated bindings, it uses a few small structs to provide compatible implementations when dealing with Objective-C APIs.

# Motivation
If you try to submit an app to the iOS App Store that contains the servo core-graphics crate, it will be rejected with the following error:

> ITMS-90338: Non-public API usage - The app references non-public symbols in Frameworks/framework_name.framework/framework_name: _CGContextSetFontSmoothingStyle. If method names in your source code match the private Apple APIs listed above, altering your method names will help prevent this app from being flagged in future submissions. In addition, note that one or more of the above APIs may be located in a static library that was included with your app. If so, they must be removed. For further information, visit the Technical Support Information at http://developer.apple.com/support/technical/

Hopefully this will get fixed, but often we’re only importing this large dependency so we can use a `CGRect`, in which case it makes sense to instead use a much smaller dependency. That’s what this crate is for.
