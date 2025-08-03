import React from 'react';

function BeverageIcon(props: React.SVGProps<SVGSVGElement>): React.ReactNode {
    return (
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            strokeWidth={1.5}
            stroke="currentColor"
            {...props}
        >
            <path strokeLinecap="round" strokeLinejoin="round" d="M8.25 6.75h7.5v10.5h-7.5z" />
            <path strokeLinecap="round" strokeLinejoin="round" d="M12 17.25v3.75M8.25 21h7.5" />
            <path strokeLinecap="round" strokeLinejoin="round" d="M12 6.75V3" />
            <path strokeLinecap="round" strokeLinejoin="round" d="M14.25 3h-4.5" />
        </svg>
    );
}

export default BeverageIcon;
