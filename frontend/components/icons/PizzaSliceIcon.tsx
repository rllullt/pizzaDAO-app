import React from 'react';

function PizzaSliceIcon(props: React.SVGProps<SVGSVGElement>): React.ReactNode {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="currentColor"
      {...props}
    >
      <path
        fillRule="evenodd"
        d="M12.982 2.583A2.25 2.25 0 0011.017 4.14L1.31 20.34a1.5 1.5 0 002.328 1.99L19.99 7.67a2.25 2.25 0 00-1.558-4.072L3.858 4.281 12.982 2.583zM14.25 10.5a1.125 1.125 0 100-2.25 1.125 1.125 0 000 2.25zM10.125 12a1.125 1.125 0 11-2.25 0 1.125 1.125 0 012.25 0zM17.25 6a1.125 1.125 0 100-2.25 1.125 1.125 0 000 2.25z"
        clipRule="evenodd"
      />
    </svg>
  );
}

export default PizzaSliceIcon;
