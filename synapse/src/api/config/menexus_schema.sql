CREATE DATABASE  IF NOT EXISTS `menexus_schema` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;
USE `menexus_schema`;
-- MySQL dump 10.13  Distrib 8.0.40, for macos14 (x86_64)
--
-- Host: localhost    Database: menexus_schema
-- ------------------------------------------------------
-- Server version	9.3.0

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `ConversationParticipants`
--

DROP TABLE IF EXISTS `ConversationParticipants`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `ConversationParticipants` (
                                            `conversation_id` bigint unsigned NOT NULL,
                                            `user_id` bigint unsigned NOT NULL,
                                            PRIMARY KEY (`conversation_id`,`user_id`),
                                            CONSTRAINT `ConversationParticipants_Conversations_conversation_id_fk` FOREIGN KEY (`conversation_id`) REFERENCES `Conversations` (`conversation_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `Conversations`
--

DROP TABLE IF EXISTS `Conversations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Conversations` (
                                 `conversation_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                                 `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                                 `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                                 PRIMARY KEY (`conversation_id`)
) ENGINE=InnoDB AUTO_INCREMENT=117 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `Followers`
--

DROP TABLE IF EXISTS `Followers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Followers` (
                             `follower_public_key` varchar(66) NOT NULL,
                             `followed_public_key` varchar(66) NOT NULL,
                             `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `Messages`
--

DROP TABLE IF EXISTS `Messages`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Messages` (
                            `conversation_id` bigint unsigned NOT NULL,
                            `message_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                            `sender_id` bigint unsigned NOT NULL,
                            `receiver_id` bigint unsigned NOT NULL,
                            `content` text,
                            `media_url` varchar(255) DEFAULT NULL,
                            `is_read` tinyint DEFAULT '0',
                            `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                            `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                            PRIMARY KEY (`message_id`),
                            KEY `Messages_Conversations_conversation_id_fk` (`conversation_id`),
                            CONSTRAINT `Messages_Conversations_conversation_id_fk` FOREIGN KEY (`conversation_id`) REFERENCES `Conversations` (`conversation_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=266 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `Notifications`
--

DROP TABLE IF EXISTS `Notifications`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Notifications` (
                                 `notification_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                                 `public_key` varchar(66) NOT NULL,
                                 `actor_id` bigint unsigned NOT NULL,
                                 `resource_type` enum('POST','COMMENT','FOLLOW') NOT NULL,
                                 `resource_id` int NOT NULL,
                                 `action` varchar(50) NOT NULL,
                                 `summary` varchar(255) NOT NULL,
                                 `is_read` tinyint NOT NULL DEFAULT '0',
                                 `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                 PRIMARY KEY (`notification_id`)
) ENGINE=InnoDB AUTO_INCREMENT=358 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `PostComments`
--

DROP TABLE IF EXISTS `PostComments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `PostComments` (
                                `comment_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                                `public_key` varchar(66) NOT NULL,
                                `resource_id` bigint unsigned NOT NULL,
                                `resource_type` varchar(50) NOT NULL,
                                `content` text NOT NULL,
                                `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                                PRIMARY KEY (`comment_id`),
                                KEY `fk_post_id_idx` (`resource_id`),
                                CONSTRAINT `fk_post_id` FOREIGN KEY (`resource_id`) REFERENCES `Posts` (`post_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=172 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `Posting`
--

DROP TABLE IF EXISTS `Posts`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Posts` (
                         `post_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                         `public_key` varchar(66) NOT NULL,
                         `content` text,
                         `media_url` varchar(255) DEFAULT NULL,
                         `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                         `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                         PRIMARY KEY (`post_id`)
) ENGINE=InnoDB AUTO_INCREMENT=99 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `Reactions`
--

DROP TABLE IF EXISTS `Reactions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Reactions` (
                             `reaction_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                             `user_id` bigint unsigned NOT NULL,
                             `resource_id` bigint unsigned NOT NULL,
                             `resource_type` varchar(50) NOT NULL,
                             `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                             PRIMARY KEY (`reaction_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `Themes`
--

DROP TABLE IF EXISTS `Themes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Themes` (
                          `theme_id` bigint NOT NULL AUTO_INCREMENT,
                          `name` varchar(100) NOT NULL,
                          `description` text,
                          `css` text,
                          `animations` json DEFAULT NULL,
                          `created_by` bigint unsigned NOT NULL,
                          `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                          `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                          PRIMARY KEY (`theme_id`),
                          UNIQUE KEY `name_UNIQUE` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `UserSettings`
--

DROP TABLE IF EXISTS `UserSettings`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `UserSettings` (
                                `user_id` bigint unsigned NOT NULL,
                                `theme_id` bigint unsigned DEFAULT NULL,
                                `use_custom_css` tinyint unsigned NOT NULL DEFAULT '0',
                                `autoplay_music` tinyint unsigned DEFAULT '0',
                                `show_online_status` tinyint unsigned DEFAULT '1',
                                `notifications_json` json DEFAULT NULL,
                                `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                                `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                                PRIMARY KEY (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2025-06-18  2:44:16
