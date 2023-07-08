CREATE TABLE text_content (
    id serial primary key not null,
    content text not null,
    content_type text not null,
    src text
);

CREATE TABLE image (
    id serial primary key not null,
    uri text not null,
    title text,
    website_link text,
    width integer,
    height integer,
    description text
);

CREATE TABLE category (
    id serial primary key not null,
    term text unique not null,
    label text
);

CREATE TABLE person (
    id serial primary key not null,
    name text not null,
    uri text,
    email text
);

CREATE TABLE channel (
    id text primary key not null,
    rss_link text not null,
    website_link text not null,
    title text not null,
    description_text_id integer references text_content(id),
    language text,
    logo_id integer references image(id),
    icon_id integer references image(id)
);

CREATE TABLE channel_category (
    channel_id text references channel(id) ON DELETE CASCADE,
    category_id integer references category(id) ON DELETE CASCADE,
    CONSTRAINT channel_category_pk PRIMARY KEY(channel_id, category_id)
);

CREATE TABLE channel_author (
    channel_id text references channel(id) ON DELETE CASCADE,
    person_id integer references person(id) ON DELETE CASCADE,
    CONSTRAINT channel_author_pk PRIMARY KEY(channel_id, person_id)
);

CREATE TABLE channel_contributor (
    channel_id text references channel(id) ON DELETE CASCADE,
    person_id integer references person(id) ON DELETE CASCADE,
    CONSTRAINT channel_contributor_pk PRIMARY KEY(channel_id, person_id)
);

CREATE TABLE content (
    id serial primary key not null,
    body text,
    content_type text,
    length bigint,
    src text
);

CREATE TABLE media_object (
    id serial primary key not null,
    duration interval,

    -- One media attachment for now
    url text,
    content_type text,
    height integer,
    width integer,
    size bigint
);

CREATE TABLE episode (
    id text primary key not null,
    channel_id text references channel(id) ON DELETE CASCADE not null ,
    published timestamptz not null,
    title text not null,
    website_link text not null,
    content_id integer references content(id),
    summary_text_id integer references text_content(id),
    media_object_id integer references media_object(id)
);

CREATE TABLE episode_category (
    episode_id text references episode(id) ON DELETE CASCADE,
    category_id integer references category(id) ON DELETE CASCADE,
    CONSTRAINT episode_category_pk PRIMARY KEY(episode_id, category_id)
);

CREATE TABLE episode_author (
    episode_id text references episode(id) ON DELETE CASCADE,
    person_id integer references person(id) ON DELETE CASCADE,
    CONSTRAINT episode_author_pk PRIMARY KEY(episode_id, person_id)
);
