import React from 'react';

function SoroswapIcon(props: React.SVGProps<SVGSVGElement>): React.ReactNode {
  return (
    <svg 
      xmlns="http://www.w3.org/2000/svg" 
      fill="none" 
      viewBox="0 0 24 24" 
      strokeWidth={1.5} 
      stroke="currentColor" 
      {...props}
    >
      <path strokeLinecap="round" strokeLinejoin="round" d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h12.75m3.75-9L21 7.5m0 0L16.5 12M21 7.5H8.25" />
    </svg>
  );
}

export default SoroswapIcon;