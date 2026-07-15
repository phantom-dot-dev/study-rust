import React from 'react';
import { Heart } from 'lucide-react';

export default function Footer() {
  return (
    <footer className="w-full py-8 border-t border-white/5 bg-black/10 mt-auto">
      <div className="max-w-6xl mx-auto px-6 flex flex-col md:flex-row items-center justify-between gap-4">
        <p className="text-sm text-gray-500">
          &copy; {new Date().getFullYear()} AntigravityChat. All rights reserved.
        </p>
        <p className="text-sm text-gray-500 flex items-center gap-1.5">
          Made with <Heart className="w-4 h-4 text-pink-500 fill-pink-500" /> for Rust & React pairing
        </p>
      </div>
    </footer>
  );
}
