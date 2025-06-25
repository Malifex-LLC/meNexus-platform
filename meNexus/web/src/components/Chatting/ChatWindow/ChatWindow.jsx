// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import ChatMessage from "../ChatMessage/ChatMessage.jsx";

const ChatWindow = () => {
    const publicKey = "03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92"

    const messages = [
        {
            "displayName": "The Architect",
            "handle": "architect",
            "publicKey": "03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92",
            "profilePicture": "/uploads/profile_pictures/1750299923820-49749251_10212372218974723_3938094030969110528_o (1).jpg",
            "content": "Hey everyone, welcome to the new #general chat!",
            "createdAt": "2025-06-22T14:00:00Z"
        },
        {
            "displayName": "Heavenlyy",
            "handle": "heavenlyy.art",
            "publicKey": "0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1",
            "profilePicture": "/uploads/profile_pictures/1750297581582-heaven_profile_picture.jpg",
            "content": "Hi Architect 👋 — digging the dark theme already.",
            "createdAt": "2025-06-22T14:00:48Z"
        },
        {
            "displayName": "meNexus Admin",
            "handle": "meNexus",
            "publicKey": "0322fbd78cc138d265fd083e9cfd7ecdafa64747bfe41873513a76eda1267404cf",
            "profilePicture": "/uploads/profile_pictures/1750299892800-menexus_logo.jpeg",
            "content": "Glad you like it! Let us know if anything feels off.",
            "createdAt": "2025-06-22T14:01:15Z"
        },
        {
            "displayName": "The Architect",
            "handle": "architect",
            "publicKey": "03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92",
            "profilePicture": "/uploads/profile_pictures/1750299923820-49749251_10212372218974723_3938094030969110528_o (1).jpg",
            "content": "First impressions: fast load times, smooth scrolling. Nice work.",
            "createdAt": "2025-06-22T14:02:02Z"
        },
        {
            "displayName": "Heavenlyy",
            "handle": "heavenlyy.art",
            "publicKey": "0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1",
            "profilePicture": "/uploads/profile_pictures/1750297581582-heaven_profile_picture.jpg",
            "content": "Test message #1. Just making sure the chat counter updates.",
            "createdAt": "2025-06-22T14:02:24Z"
        },
        {
            "displayName": "Heavenlyy",
            "handle": "heavenlyy.art",
            "publicKey": "0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1",
            "profilePicture": "/uploads/profile_pictures/1750297581582-heaven_profile_picture.jpg",
            "content": "…and a follow-up to see multi-line handling.\nSecond line ✔️",
            "createdAt": "2025-06-22T14:02:46Z"
        },
        {
            "displayName": "meNexus Admin",
            "handle": "meNexus",
            "publicKey": "0322fbd78cc138d265fd083e9cfd7ecdafa64747bfe41873513a76eda1267404cf",
            "profilePicture": "/uploads/profile_pictures/1750299892800-menexus_logo.jpeg",
            "content": "Multi-line looks good in my viewport.",
            "createdAt": "2025-06-22T14:03:10Z"
        },
        {
            "displayName": "The Architect",
            "handle": "architect",
            "publicKey": "03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92",
            "profilePicture": "/uploads/profile_pictures/1750299923820-49749251_10212372218974723_3938094030969110528_o (1).jpg",
            "content": "Emoji test: 🎉 🚀 😎",
            "createdAt": "2025-06-22T14:03:44Z"
        },
        {
            "displayName": "Heavenlyy",
            "handle": "heavenlyy.art",
            "publicKey": "0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1",
            "profilePicture": "/uploads/profile_pictures/1750297581582-heaven_profile_picture.jpg",
            "content": "👍🏽 Emojis render fine here too.",
            "createdAt": "2025-06-22T14:04:05Z"
        },
        {
            "displayName": "meNexus Admin",
            "handle": "meNexus",
            "publicKey": "0322fbd78cc138d265fd083e9cfd7ecdafa64747bfe41873513a76eda1267404cf",
            "profilePicture": "/uploads/profile_pictures/1750299892800-menexus_logo.jpeg",
            "content": "Logging an internal note: add reactions next sprint.",
            "createdAt": "2025-06-22T14:04:36Z"
        },
        {
            "displayName": "The Architect",
            "handle": "architect",
            "publicKey": "03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92",
            "profilePicture": "/uploads/profile_pictures/1750299923820-49749251_10212372218974723_3938094030969110528_o (1).jpg",
            "content": "Quick question: what’s the max message length before truncation?",
            "createdAt": "2025-06-22T14:05:07Z"
        },
        {
            "displayName": "meNexus Admin",
            "handle": "meNexus",
            "publicKey": "0322fbd78cc138d265fd083e9cfd7ecdafa64747bfe41873513a76eda1267404cf",
            "profilePicture": "/uploads/profile_pictures/1750299892800-menexus_logo.jpeg",
            "content": "Currently 2 000 characters; hard limit at DB level.",
            "createdAt": "2025-06-22T14:05:41Z"
        },
        {
            "displayName": "Heavenlyy",
            "handle": "heavenlyy.art",
            "publicKey": "0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1",
            "profilePicture": "/uploads/profile_pictures/1750297581582-heaven_profile_picture.jpg",
            "content": "Nice. That covers blog-length rants 😂",
            "createdAt": "2025-06-22T14:06:03Z"
        },
        {
            "displayName": "The Architect",
            "handle": "architect",
            "publicKey": "03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92",
            "profilePicture": "/uploads/profile_pictures/1750299923820-49749251_10212372218974723_3938094030969110528_o (1).jpg",
            "content": "Markdown italics *test* and **bold** test.",
            "createdAt": "2025-06-22T14:06:38Z"
        },
        {
            "displayName": "meNexus Admin",
            "handle": "meNexus",
            "publicKey": "0322fbd78cc138d265fd083e9cfd7ecdafa64747bfe41873513a76eda1267404cf",
            "profilePicture": "/uploads/profile_pictures/1750299892800-menexus_logo.jpeg",
            "content": "Formatting parses correctly on my end.",
            "createdAt": "2025-06-22T14:07:02Z"
        },
        {
            "displayName": "Heavenlyy",
            "handle": "heavenlyy.art",
            "publicKey": "0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1",
            "profilePicture": "/uploads/profile_pictures/1750297581582-heaven_profile_picture.jpg",
            "content": "Can we drop a link? https://menexus.dev",
            "createdAt": "2025-06-22T14:07:19Z"
        },
        {
            "displayName": "The Architect",
            "handle": "architect",
            "publicKey": "03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92",
            "profilePicture": "/uploads/profile_pictures/1750299923820-49749251_10212372218974723_3938094030969110528_o (1).jpg",
            "content": "Clickable on desktop ✅",
            "createdAt": "2025-06-22T14:07:45Z"
        },
        {
            "displayName": "meNexus Admin",
            "handle": "meNexus",
            "publicKey": "0322fbd78cc138d265fd083e9cfd7ecdafa64747bfe41873513a76eda1267404cf",
            "profilePicture": "/uploads/profile_pictures/1750299892800-menexus_logo.jpeg",
            "content": "Mobile detect passes too.",
            "createdAt": "2025-06-22T14:08:11Z"
        },
        {
            "displayName": "Heavenlyy",
            "handle": "heavenlyy.art",
            "publicKey": "0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1",
            "profilePicture": "/uploads/profile_pictures/1750297581582-heaven_profile_picture.jpg",
            "content": "Alright, I’ll hop to #memes — see y’all there!",
            "createdAt": "2025-06-22T14:08:38Z"
        },
        {
            "displayName": "The Architect",
            "handle": "architect",
            "publicKey": "03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92",
            "profilePicture": "/uploads/profile_pictures/1750299923820-49749251_10212372218974723_3938094030969110528_o (1).jpg",
            "content": "Copy that. Switching channels.",
            "createdAt": "2025-06-22T14:08:55Z"
        },
        {
            "displayName": "meNexus Admin",
            "handle": "meNexus",
            "publicKey": "0322fbd78cc138d265fd083e9cfd7ecdafa64747bfe41873513a76eda1267404cf",
            "profilePicture": "/uploads/profile_pictures/1750299892800-menexus_logo.jpeg",
            "content": "Thanks for testing! I’ll compile these notes.",
            "createdAt": "2025-06-22T14:09:21Z"
        },
        {
            "displayName": "Heavenlyy",
            "handle": "heavenlyy.art",
            "publicKey": "0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1",
            "profilePicture": "/uploads/profile_pictures/1750297581582-heaven_profile_picture.jpg",
            "content": "One last ping before I go 🚪",
            "createdAt": "2025-06-22T14:09:46Z"
        },
        {
            "displayName": "The Architect",
            "handle": "architect",
            "publicKey": "03f507f13ad44b718fdb3db0602645c845d58cba7abd521f4005cadb60044caa92",
            "profilePicture": "/uploads/profile_pictures/1750299923820-49749251_10212372218974723_3938094030969110528_o (1).jpg",
            "content": "All good here.",
            "createdAt": "2025-06-22T14:10:02Z"
        },
        {
            "displayName": "meNexus Admin",
            "handle": "meNexus",
            "publicKey": "0322fbd78cc138d265fd083e9cfd7ecdafa64747bfe41873513a76eda1267404cf",
            "profilePicture": "/uploads/profile_pictures/1750299892800-menexus_logo.jpeg",
            "content": "Chat cleared for production ✨",
            "createdAt": "2025-06-22T14:10:25Z"
        },
        {
            "displayName": "Heavenlyy",
            "handle": "heavenlyy.art",
            "publicKey": "0254c54d0b7521daafa66f701cf8d59dce05a3c5126e69aa6f7bf62119936b03d1",
            "profilePicture": "/uploads/profile_pictures/1750297581582-heaven_profile_picture.jpg",
            "content": "Awesome work, team 🙌",
            "createdAt": "2025-06-22T14:10:47Z"
        }
    ]



    return (
        <div className={'bg-background'}>
            <div className="flex-1 overflow-y-auto p-4  space-y-4 shadow-2xl ">
                {messages.map((msg, i) => {
                    const isOwner = msg.publicKey === publicKey;

                    return (
                        <div key={i} className={`flex ${isOwner ? "justify-end" : "justify-start"}`}>
                            <ChatMessage message={msg} isOwner={isOwner} />
                        </div>
                    );
                })}
            </div>
        </div>
    );
}

export default ChatWindow;