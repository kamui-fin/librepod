# LibrePod

[![Licence](https://img.shields.io/github/license/kamui-fin/librepod?style=for-the-badge)](./LICENSE)
![Figma](https://img.shields.io/badge/figma-%23F24E1E.svg?style=for-the-badge&logo=figma&logoColor=white)

A privacy-friendly, self-hosted podcast aggregator optimized for streaming. Although quite a lofty goal, the idea is to work towards building something similar to a PocketCasts open-source replacement in the long-term.

On top of the standard functionality, I'm also interested in developing additional features for an optimized [passive listening](https://refold.la/roadmap/stage-0/c/passive-listening) experience for language learners.

### Features

Currently, a basic MVP is being developed with the most useful and familiar features such as:

- Modern and minimal UI interface with PWA experience
- User system, where each user gets to maintain their own customized feed and preferences
- Add, remove, and list subscriptions
- View channels and episode content
- Automaticaly generate and update customized feed hourly
  - Have multiple auto-generated “views” within date ranges (e.g. today, past week, etc.)
- Stream episodes over HTTP efficiently
  - Play “views” (filter out already played)
  - Shuffle option
- Track listening history and skip played episodes
- Web player & queue with persistent playback state across sessions
- OPML import & export
- Automatically managed downloads

## Sample Previews

Note: the following UI previews are not yet finalized and will undergo further improvements.

<img src="./images/demo_login.png" width="400" />
<img src="./images/demo_feed.png" width="700" />
<img src="./images/pallete.svg" width="300" />

### Architecture

**API**: Rust, Axum & Tower HTTP, sqlx

**Stores**: PostgreSQL, Redis

**Web Client**: React, Vite, Typescript, Sass

Librepod consists of a typical server-client architecture, with the server performing the majority of the heavylifting (i.e. feed polling, caching, etc.) for a variety of reasons:

- Centralizing the business logic ensures a standardized and consistent approach across all client applications.
  - Developers can focus on integrating the API without worrying about the underlying intricacies of podcast aggregation
- Blazing-fast client-side experience
- Server-client architecture allows seamless feed synchronization between multiple devices
- Enables state saving and reconciliation from different clients
- Easier maintenance and updates

Cookie-based sessions are used for authentication with argon2 hashing.

Librepod supports parsing Atom, JSON, RSS0, RSS1, and RSS2 feeds. HTTP results are cached approriately according to RFC 7234 in Redis for quicker polling.

If scaling up becomes necessary, I'm considering spinning up a separate micro-service for feed generation specifically.

### API

The Rust-based LibrePod API aims to provide developers with a centralized and comprehensive interface, allowing them to integrate the platform seamlessly into their own applications and services.
The key concept is to encapsulate all podcast-related business logic within a "black box" model on the server side.
Clients will interact with the API to fetch data and perform actions, abstracting away the complexities of podcast aggregation, caching, and other operations.

While the API is still changing rapidly and swagger documentation will soon be written, I want to document a few important things
that may cause some confusion.

#### Channels vs Subscriptions vs Feed

A `Feed` in Librepod refers to the overall latest **aggregation** of all episodes collected from a user's subscriptions. Each user
has their own feed.

A `Channel` is an **immutable** entity representing a podcast source pulled from RSS.

A user may _subscribe_ to a `Channel` creating a `Subscription` relation between the `User` and the `Channel`.
Even if they unsubscribe, the `Channel` remains on the server.

Channels and subscriptions are distinguished as a caching mechanism, proving to be useful if multiple users exist on a single Librepod instance and potential overlaps in subscriptions.
Remember, librepod was designed with **scalability** in mind.

### Future Roadmap

- Introduce Web Sub support, as it is substantially more efficient than manual polling for the feeds that support it
- Load data from PodcastIndex for features like searching feeds, viewing the trending ones per country, etc.
  - Recommendation system (crowdsourced?)
- Develop a Linux command-line client that can be headlessly operated using IPC
  - Potentially hooked up with MPD
  - Features like play on boot, auto resume/pause based on other sound activity
- Attach priorities to certain subscriptions
- Document API routes
- Language learning features
  - Transcript integration
  - Adjustable playback speed
  - Language filter
- User analytics to provide insights on listening habits
- Collaborative playlists
- Social sharing

## Development Environment

As LibrePod is currently in its MVP stage, the following instructions are for development and testing only. Before proceeding, clone the repository:

```bash
git clone https://github.com/kamui-fin/librepod
cd librepod
```

Configure the required environment variables for the server:
| Variable | Example |
|--------------- | --------------- |
| DATABASE_URL | postgres://postgres:postgres@localhost/librepod |
| REDIS_URL | redis://127.0.0.1 |

To start the API on `http://localhost:3000`:

```bash
cargo install sqlx-cli
sqlx migrate run
cargo run
```

Set the API url for the frontend `.env`:
| Variable | Example |
|--------------- | --------------- |
| VITE_API_URL | http://localhost:3000 |

To start the frontend on [`http://localhost:5173`](`http://localhost:5173`):

```bash
cd client
npm install --save-dev # only first time
npm run dev
```

## Contributions

All contributions are greatly appreciated, including but not limited to PRs for UI and API improvements, bug fixes, design upgrades, feature ideas, advice, etc.

## License

LibrePod is licensed under the GPLv3 License.
