import React from 'react';
import ArrowRightIcon from './icons/ArrowRightIcon';

interface AdCardProps {
    icon: React.ReactNode;
    title: string;
    onClick?: () => void;
}

function AdCard({ icon, title, onClick }: AdCardProps): React.ReactNode {
    return (
        <button
            onClick={onClick}
            disabled={true} // Non-functional for now
            className="w-full bg-zinc-900/70 p-4 rounded-xl flex items-center space-x-4 text-left transition-all duration-200 hover:bg-zinc-700/80 focus:outline-none focus:ring-2 focus:ring-yellow-500/50 border border-zinc-700 disabled:cursor-not-allowed"
        >
            <div className="flex-shrink-0 bg-zinc-800 p-3 rounded-full">
                {icon}
            </div>
            <div className="flex-grow">
                <p className="font-bold text-white text-md">{title}</p>
            </div>
            <ArrowRightIcon className="w-5 h-5 text-zinc-500" />
        </button>
    );
}

export default AdCard;