-- Your SQL goes here
-- CreateEnum
CREATE TYPE "website_status" AS ENUM (
    'Up',
    'Down',
    'Unknown'
);

-- CreateTable
CREATE TABLE "user" (
    "id" text NOT NULL,
    "username" text NOT NULL,
    "password" text NOT NULL,
    CONSTRAINT "user_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "website" (
    "id" text NOT NULL,
    "url" text NOT NULL,
    "time_added" timestamp(3) NOT NULL,
    "user_id" text NOT NULL,
    CONSTRAINT "Website_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "website_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "region" (
    "id" text NOT NULL,
    "name" text NOT NULL,
    CONSTRAINT "Region_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "website_tick" (
    "id" text NOT NULL,
    "response_time_ms" integer NOT NULL,
    "status" "website_status" NOT NULL,
    "region_id" text NOT NULL,
    "website_id" text NOT NULL,
    "created_at" timestamp(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "website_tick_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "website_tick_website_id_fkey" FOREIGN KEY ("website_id") REFERENCES "website" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
