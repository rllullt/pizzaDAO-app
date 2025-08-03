import React from 'react';
import PizzaSliceIcon from './icons/PizzaSliceIcon';

function Spinner(): React.ReactNode {
  return (
    <div role="status">
      <PizzaSliceIcon className="w-5 h-5 animate-spin text-white" />
      <span className="sr-only">Loading...</span>
    </div>
  );
}

export default Spinner;
