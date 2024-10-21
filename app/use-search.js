'use client'
import { useState } from 'react';
import { invoke } from "@tauri-apps/api/core"
import AlertContent from "./AlertContent"
import { useAlert } from '@/components/ui/custom-alert-dialog';
export const useSearch = (searchCallback) => {
  const [keywords, setKeywords] = useState('');
  const [startPage, setStartPage] = useState(1);
  const [maxCount, setMaxCount] = useState(10);
  const openAlert = useAlert();

  const handleSearch = async () => {
    console.log ('handleSearch')
    const base64Qrcode = await invoke("run_crawler",{
      keywords,
      startPage,
      maxCount,
    });
    openAlert(AlertContent, { message: base64Qrcode });
    
    console.log(base64Qrcode, 'base64Qrcode');
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