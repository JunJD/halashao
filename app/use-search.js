'use client'
import { useState } from 'react';

export const useSearch = (searchCallback) => {
  const [keywords, setKeywords] = useState('');
  const [startPage, setStartPage] = useState(1);
  const [maxCount, setMaxCount] = useState(10);

  const handleSearch = () => {
    searchCallback(keywords, startPage, maxCount);
  };

  return {
    keywords,
    setKeywords,
    startPage,
    setStartPage,
    maxCount,
    setMaxCount,
    handleSearch,
  };
};