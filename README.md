# linkleaner [![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/) [![Built with Garnix](https://img.shields.io/static/v1?label=Built%20with&message=Garnix&color=blue&style=flat&logo=nixos&link=https://garnix.io&labelColor=111212)](https://garnix.io)

Telegram bot to replace social media links with their improved preview variants. Supported platforms:

- Twitter: [FixTweet](https://github.com/FixTweet/FixTweet)
- Instagram: [InstaFix](https://github.com/Wikidepia/InstaFix) (behind the `ddinstagram` feature)
- Accelerated Mobile Pages (AMP): [AmputatorBot](https://www.amputatorbot.com/)
- YouTube Shorts: In-process transform (changes `/shorts/` URLs to regular video player)

### Running

- Copy `.env.sample` as `.env` and edit with the necessary details
- Use `cargo run` to start the bot
