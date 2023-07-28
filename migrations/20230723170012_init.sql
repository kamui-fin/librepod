CREATE TABLE channel (
    id uuid primary key not null,
    title text not null,
    rss_link text not null,
    website_link text not null,
    author text,
    description text,
    tags text -- comma separated
    -- image stored in fs, images/[channel_id]
);

CREATE TABLE episode (
    id uuid primary key not null,
    channel_id uuid references channel(id) ON DELETE CASCADE not null ,
    website_link text not null,
    published timestamptz not null,
    title text not null,
    audio_link text not null,
    description text,
    content text,
    tags text
);

-- table name user is reserved
CREATE TABLE account (
    id uuid not null primary key,
    name varchar(20) unique not null,
    email varchar unique not null,
    password text not null,
    salt bytea not null,
    created_at timestamptz DEFAULT now()
);

CREATE TABLE user_subscriptions (
    user_id uuid references account(id) ON DELETE CASCADE,
    channel_id uuid references channel(id) ON DELETE CASCADE,
    CONSTRAINT user_subscription_pk PRIMARY KEY(user_id, channel_id)
);

CREATE TABLE user_watch_history (
    user_id uuid references account(id) ON DELETE CASCADE,
    episode_id uuid references episode(id) ON DELETE CASCADE,
    CONSTRAINT user_watch_history_pk PRIMARY KEY(user_id, episode_id)
);
