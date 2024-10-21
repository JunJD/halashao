'use client'
import React, { useState, useCallback, createContext, useContext } from 'react';
import { createPortal } from 'react-dom';
import { AlertDialog, AlertDialogContent, AlertDialogOverlay } from './alert-dialog';

const AlertContext = createContext(null);

const CustomAlertDialog = ({ isOpen, onClose, children }) => {
  return createPortal(
    <AlertDialog open={isOpen} onOpenChange={onClose}>
      <AlertDialogOverlay />
      <AlertDialogContent>
        {children}
      </AlertDialogContent>
    </AlertDialog>,
    document.body
  );
};

export const AlertProvider = ({ children }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [content, setContent] = useState(null);

  const openAlert = useCallback((Component, props = {}) => {
    const closeAlert = () => {
      setIsOpen(false);
    };

    setContent(<Component {...props} onClose={closeAlert} />);
    setIsOpen(true);
  }, []);

  return (
    <AlertContext.Provider value={{ openAlert }}>
      {children}
      {isOpen && (
        <CustomAlertDialog isOpen={isOpen} onClose={() => setIsOpen(false)}>
          {content}
        </CustomAlertDialog>
      )}
    </AlertContext.Provider>
  );
};

export const useAlert = () => {
  const context = useContext(AlertContext);
  if (!context) {
    throw new Error('useAlert must be used within an AlertProvider');
  }
  return context.openAlert;
};
