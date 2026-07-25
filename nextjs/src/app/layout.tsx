import type { Metadata } from 'next';
import "./globals.css"

export const metadata: Metadata = {
  title: 'Connect Four',
  description: 'Connect Four game powered by Next.js and Rust/WASM',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html>
      <body>{children}</body>
    </html>
  );
}
