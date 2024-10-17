'use client'
import { useState } from 'react';
import { Comment, mockComments } from './mock-data';

export const useComments = () => {
  const [comments, setComments] = useState([]);
  const [isLoading, setIsLoading] = useState(false);
  const [progress, setProgress] = useState(0);

  const searchComments = (keywords, startPage, maxCount) => {
    setIsLoading(true);
    setProgress(0);

    const interval = setInterval(() => {
      setProgress(prev => (prev >= 100 ? 100 : prev + 1));
    }, 20);

    setTimeout(() => {
      const filteredComments = mockComments.filter(comment => 
        comment.content.toLowerCase().includes(keywords.toLowerCase()) || 
        comment.replies.some(reply => reply.content.toLowerCase().includes(keywords.toLowerCase()))
      ).slice((startPage - 1) * maxCount, startPage * maxCount);

      setComments(filteredComments);
      setIsLoading(false);
      clearInterval(interval);
      setProgress(100);
    }, 2000);
  };

  return { comments, isLoading, progress, searchComments };
};