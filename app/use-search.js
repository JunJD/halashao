'use client'
import { useState } from 'react';
import { invoke } from "@tauri-apps/api/core"
export const useSearch = (searchCallback) => {
  const [keywords, setKeywords] = useState('');
  const [startPage, setStartPage] = useState(1);
  const [maxCount, setMaxCount] = useState(10);

  const handleSearch = async () => {
    await invoke("run_crawler");
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