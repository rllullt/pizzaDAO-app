import React from 'react';

function WearableTokenIcon(props: React.SVGProps<SVGSVGElement>): React.ReactNode {
  return (
    <svg 
        xmlns="http://www.w3.org/2000/svg" 
        fill="none" 
        viewBox="0 0 24 24" 
        strokeWidth={1.5} 
        stroke="currentColor"
        {...props}
    >
        <path strokeLinecap="round" strokeLinejoin="round" d="M15.362 5.214A8.252 8.252 0 0112 21 8.25 8.25 0 016 17.857 8.25 8.25 0 018.638 5.214 8.252 8.252 0 0112 3a8.252 8.252 0 013.362 2.214zM12 12.75a.75.75 0 000 1.5h.008a.75.75 0 000-1.5H12z" />
        <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 21h16.5" />
    </svg>
  );
}

export default WearableTokenIcon;
