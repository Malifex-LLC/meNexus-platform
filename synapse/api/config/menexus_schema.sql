-- MySQL dump 10.13  Distrib 9.0.1, for macos14.7 (x86_64)
--
-- Host: 127.0.0.1    Database: menexus_schema
-- ------------------------------------------------------
-- Server version	9.0.1

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `Authentication`
--

DROP TABLE IF EXISTS `Authentication`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Authentication` (
                                  `auth_id` bigint NOT NULL AUTO_INCREMENT,
                                  `user_id` bigint unsigned NOT NULL,
                                  `email` varchar(255) NOT NULL,
                                  `hashed_password` varchar(255) DEFAULT NULL,
                                  `auth_provider` varchar(50) DEFAULT 'local',
                                  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                                  PRIMARY KEY (`auth_id`),
                                  UNIQUE KEY `email_UNIQUE` (`email`),
                                  KEY `fk_authentication_user_id_idx` (`user_id`),
                                  CONSTRAINT `fk_authentication_user_id` FOREIGN KEY (`user_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=17 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Authentication`
--

LOCK TABLES `Authentication` WRITE;
/*!40000 ALTER TABLE `Authentication` DISABLE KEYS */;
INSERT INTO `Authentication` VALUES (1,1,'menexus@mail.com','$2b$10$obUDhrFqItVqnqN0LxMaZOYEMej3QdhVWZRox5ZayGlRebEKUvIYe','local','2024-12-18 03:53:11');
/*!40000 ALTER TABLE `Authentication` ENABLE KEYS */;
UNLOCK TABLES;

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
                                            KEY `ConversationParticipants_Users_user_id_fk` (`user_id`),
                                            CONSTRAINT `ConversationParticipants_Conversations_conversation_id_fk` FOREIGN KEY (`conversation_id`) REFERENCES `Conversations` (`conversation_id`) ON DELETE CASCADE ON UPDATE CASCADE,
                                            CONSTRAINT `ConversationParticipants_Users_user_id_fk` FOREIGN KEY (`user_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ConversationParticipants`
--

LOCK TABLES `ConversationParticipants` WRITE;
/*!40000 ALTER TABLE `ConversationParticipants` DISABLE KEYS */;
/*!40000 ALTER TABLE `ConversationParticipants` ENABLE KEYS */;
UNLOCK TABLES;

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
) ENGINE=InnoDB AUTO_INCREMENT=82 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Conversations`
--

LOCK TABLES `Conversations` WRITE;
/*!40000 ALTER TABLE `Conversations` DISABLE KEYS */;
/*!40000 ALTER TABLE `Conversations` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Followers`
--

DROP TABLE IF EXISTS `Followers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Followers` (
                             `follower_id` bigint unsigned NOT NULL,
                             `followed_id` bigint unsigned NOT NULL,
                             `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                             PRIMARY KEY (`follower_id`,`followed_id`),
                             KEY `fk_follower_user_id_idx` (`follower_id`),
                             KEY `fk_followed_user_id_idx` (`followed_id`),
                             CONSTRAINT `fk_followed_user_id` FOREIGN KEY (`followed_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE,
                             CONSTRAINT `fk_follower_user_id` FOREIGN KEY (`follower_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Followers`
--

LOCK TABLES `Followers` WRITE;
/*!40000 ALTER TABLE `Followers` DISABLE KEYS */;
/*!40000 ALTER TABLE `Followers` ENABLE KEYS */;
UNLOCK TABLES;

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
                            KEY `fk_sender_user_id_idx` (`sender_id`),
                            KEY `fk_receiver_user_id_idx` (`receiver_id`),
                            KEY `Messages_Conversations_conversation_id_fk` (`conversation_id`),
                            CONSTRAINT `fk_receiver_user_id` FOREIGN KEY (`receiver_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE,
                            CONSTRAINT `fk_sender_user_id` FOREIGN KEY (`sender_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE,
                            CONSTRAINT `Messages_Conversations_conversation_id_fk` FOREIGN KEY (`conversation_id`) REFERENCES `Conversations` (`conversation_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=161 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Messages`
--

LOCK TABLES `Messages` WRITE;
/*!40000 ALTER TABLE `Messages` DISABLE KEYS */;
/*!40000 ALTER TABLE `Messages` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Notifications`
--

DROP TABLE IF EXISTS `Notifications`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Notifications` (
                                 `notification_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                                 `user_id` bigint unsigned NOT NULL,
                                 `actor_id` bigint unsigned NOT NULL,
                                 `resource_type` enum('POST','COMMENT','FOLLOW') NOT NULL,
                                 `resource_id` int NOT NULL,
                                 `action` varchar(50) NOT NULL,
                                 `summary` varchar(255) NOT NULL,
                                 `is_read` tinyint NOT NULL DEFAULT '0',
                                 `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                 PRIMARY KEY (`notification_id`),
                                 KEY `fk_notification_user_id_idx` (`user_id`),
                                 KEY `fk_notification_actor_id_idx` (`actor_id`),
                                 CONSTRAINT `fk_notification_actor_id` FOREIGN KEY (`actor_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE,
                                 CONSTRAINT `fk_notification_user_id` FOREIGN KEY (`user_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=216 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Notifications`
--

LOCK TABLES `Notifications` WRITE;
/*!40000 ALTER TABLE `Notifications` DISABLE KEYS */;
/*!40000 ALTER TABLE `Notifications` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `PostComments`
--

DROP TABLE IF EXISTS `PostComments`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `PostComments` (
                                `comment_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                                `user_id` bigint unsigned NOT NULL,
                                `resource_id` bigint unsigned NOT NULL,
                                `resource_type` varchar(50) NOT NULL,
                                `content` text NOT NULL,
                                `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                                PRIMARY KEY (`comment_id`),
                                KEY `fk_comment_user_id_idx` (`user_id`),
                                KEY `fk_post_id_idx` (`resource_id`),
                                CONSTRAINT `fk_comment_user_id` FOREIGN KEY (`user_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE,
                                CONSTRAINT `fk_post_id` FOREIGN KEY (`resource_id`) REFERENCES `Posts` (`post_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=97 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `PostComments`
--

LOCK TABLES `PostComments` WRITE;
/*!40000 ALTER TABLE `PostComments` DISABLE KEYS */;
/*!40000 ALTER TABLE `PostComments` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Posts`
--

DROP TABLE IF EXISTS `Posts`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Posts` (
                         `post_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                         `user_id` bigint unsigned NOT NULL,
                         `content` text,
                         `media_url` varchar(255) DEFAULT NULL,
                         `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                         `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                         PRIMARY KEY (`post_id`),
                         KEY `fk_post_user_id_idx` (`user_id`),
                         CONSTRAINT `fk_post_user_id` FOREIGN KEY (`user_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=44 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Posts`
--

LOCK TABLES `Posts` WRITE;
/*!40000 ALTER TABLE `Posts` DISABLE KEYS */;
INSERT INTO `Posts` VALUES (1,1,'The migration to the new orbitdb schema has been completed! Everything appears to be operational!',NULL,'2024-12-18 03:57:46','2024-12-18 04:55:25'),(6,1,'Welcome to meNexus! This is the admin account where you can find everything related to meNexus!',NULL,'2024-12-18 04:55:30','2024-12-18 04:55:30'),(11,1,'meNexus has been updated to support custom profile pictures! You can update yours under Settings -> Profile -> Profile Picture!',NULL,'2024-12-19 02:43:21','2024-12-19 02:43:21'),(15,1,'meNexus orbitdb schema has been updated to use BIGINTs instead of INTs. This will support exponentially more users and interactions!',NULL,'2024-12-19 20:28:17','2024-12-19 20:28:17'),(18,1,'meNexus now supports Followers! Follow your favorite meNexus users to stay up-to-date with their most recent posts!',NULL,'2024-12-19 23:33:01','2024-12-19 23:33:01'),(28,1,'I\'m sure you\'ve noticed, but meNexus now supports comments! This is a huge step forward into making this a more social experience!',NULL,'2024-12-21 09:15:55','2024-12-21 09:16:01'),(29,1,'We have a long road ahead, but I genuinely believe that meNexus can blossom into something truly exciting.',NULL,'2024-12-21 09:17:08','2024-12-21 09:17:08'),(30,1,'meNexus now supports Notifications! Get notified on new followers, post comments, and more coming soon. While there is still development to be done to improve notifications, we hope you enjoy the new feature!',NULL,'2024-12-22 09:40:00','2024-12-22 09:40:00'),(42,1,'You are now able to update your account and profile information! Navigate to the Settings page...there you will find options for Account and Profile where you can update your email, password, display name, handle, and more!',NULL,'2025-01-01 00:57:57','2025-01-01 00:58:16'),(43,1,'MeNexus has just launched its new messaging system! You can now chat in realtime with anyone on meNexus! We are very excited about this feature and we will continue to develop the messaging system to support more features soon!',NULL,'2025-01-02 23:03:01','2025-01-02 23:03:01');
/*!40000 ALTER TABLE `Posts` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Profiles`
--

DROP TABLE IF EXISTS `Profiles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Profiles` (
                            `profile_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                            `user_id` bigint unsigned NOT NULL,
                            `profile_name` varchar(255) DEFAULT NULL,
                            `profile_bio` varchar(255) DEFAULT NULL,
                            `profile_location` varchar(255) DEFAULT NULL,
                            `profile_picture` varchar(255) DEFAULT NULL,
                            `profile_banner` varchar(255) DEFAULT NULL,
                            `custom_css` text,
                            `background_music` varchar(255) DEFAULT NULL,
                            `animations` json DEFAULT NULL,
                            `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
                            `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                            PRIMARY KEY (`profile_id`),
                            KEY `fk_profile_use_id_idx` (`user_id`),
                            CONSTRAINT `fk_profile_user_id` FOREIGN KEY (`user_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=17 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Profiles`
--

LOCK TABLES `Profiles` WRITE;
/*!40000 ALTER TABLE `Profiles` DISABLE KEYS */;
INSERT INTO `Profiles` VALUES (1,1,'meNexus','meNexus Admin Account','Network','/uploads/profile_pictures/1735003403353-menexus_logo.jpeg',NULL,NULL,NULL,NULL,'2024-12-24 01:04:40','2024-12-24 01:23:23');
/*!40000 ALTER TABLE `Profiles` ENABLE KEYS */;
UNLOCK TABLES;

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
                             PRIMARY KEY (`reaction_id`),
                             KEY `fk_reaction_user_id_idx` (`user_id`),
                             CONSTRAINT `fk_reaction_user_id` FOREIGN KEY (`user_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Reactions`
--

LOCK TABLES `Reactions` WRITE;
/*!40000 ALTER TABLE `Reactions` DISABLE KEYS */;
/*!40000 ALTER TABLE `Reactions` ENABLE KEYS */;
UNLOCK TABLES;

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
                          UNIQUE KEY `name_UNIQUE` (`name`),
                          KEY `fk_theme_created_by_user_id_idx` (`created_by`),
                          CONSTRAINT `fk_theme_created_by_user_id` FOREIGN KEY (`created_by`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Themes`
--

LOCK TABLES `Themes` WRITE;
/*!40000 ALTER TABLE `Themes` DISABLE KEYS */;
/*!40000 ALTER TABLE `Themes` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Users`
--

DROP TABLE IF EXISTS `Users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Users` (
                         `user_id` bigint unsigned NOT NULL AUTO_INCREMENT,
                         `handle` varchar(50) NOT NULL,
                         `display_name` varchar(100) NOT NULL,
                         `is_online` tinyint DEFAULT '0',
                         `last_active_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
                         `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
                         `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                         PRIMARY KEY (`user_id`),
                         UNIQUE KEY `handle_UNIQUE` (`handle`)
) ENGINE=InnoDB AUTO_INCREMENT=18 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Users`
--

LOCK TABLES `Users` WRITE;
/*!40000 ALTER TABLE `Users` DISABLE KEYS */;
INSERT INTO `Users` VALUES (1,'meNexus','meNexus',0,'2024-12-18 03:53:11','2024-12-18 03:53:11','2024-12-18 03:53:26');
/*!40000 ALTER TABLE `Users` ENABLE KEYS */;
UNLOCK TABLES;

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
                                PRIMARY KEY (`user_id`),
                                CONSTRAINT `fk_user_settings_user_id` FOREIGN KEY (`user_id`) REFERENCES `Users` (`user_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `UserSettings`
--

LOCK TABLES `UserSettings` WRITE;
/*!40000 ALTER TABLE `UserSettings` DISABLE KEYS */;
/*!40000 ALTER TABLE `UserSettings` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2025-01-02 17:28:24
