-- table name user is reserved
CREATE TABLE account (
    id uuid not null primary key,
    name varchar(20) unique not null,
    email varchar unique not null,
    password text not null
    salt bytea not null,
    created_at timestamptz DEFAULT now()
);

CREATE TABLE user_subscriptions (
    user_id uuid references account(id) ON DELETE CASCADE,
    channel_id text references channel(id) ON DELETE CASCADE,
    CONSTRAINT user_subscription_pk PRIMARY KEY(user_id, channel_id)
);

CREATE TABLE user_watch_history (
    user_id uuid references account(id) ON DELETE CASCADE,
    episode_id text references episode(id) ON DELETE CASCADE,
    CONSTRAINT user_watch_history_pk PRIMARY KEY(user_id, text_id)
);
