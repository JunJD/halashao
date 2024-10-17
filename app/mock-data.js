// export interface Comment {
//     comment_id: string;
//     create_time: string;
//     note_id: string;
//     content: string;
//     user_id: string;
//     nickname: string;
//     avatar: string;
//     last_modify_ts: string;
//     like_count: number;
//     replies: Comment[];
// }

export const generateMockComments = (count) => {
    const comments = [];
    for (let i = 0; i < count; i++) {
        comments.push({
            comment_id: `${i + 1}`,
            create_time: new Date(Date.now() - Math.random() * 10000000000).toISOString(),
            note_id: `${Math.floor(Math.random() * 1000)}`,
            content: `这是第 ${i + 1} 个顶级评论`,
            user_id: `user${i + 1}`,
            nickname: `用户${i + 1}`,
            avatar: `/placeholder.svg?height=40&width=40`,
            last_modify_ts: new Date(Date.now() - Math.random() * 1000000000).toISOString(),
            like_count: Math.floor(Math.random() * 100),
            replies: Array(Math.floor(Math.random() * 5)).fill(null).map((_, j) => ({
                comment_id: `${i + 1}-${j + 1}`,
                create_time: new Date(Date.now() - Math.random() * 1000000000).toISOString(),
                note_id: `${Math.floor(Math.random() * 1000)}`,
                content: `这是第 ${i + 1} 个顶级评论的第 ${j + 1} 个回复`,
                user_id: `user${i + 1}-${j + 1}`,
                nickname: `用户${i + 1}-${j + 1}`,
                avatar: `/placeholder.svg?height=40&width=40`,
                last_modify_ts: new Date(Date.now() - Math.random() * 100000000).toISOString(),
                like_count: Math.floor(Math.random() * 50),
                replies: []
            }))
        });
    }
    return comments;
};

export const mockComments = generateMockComments(50);