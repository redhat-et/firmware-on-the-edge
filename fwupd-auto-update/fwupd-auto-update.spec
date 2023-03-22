Name: fwupd-auto-update

Version: 1.0.0
Release: 1

Summary: fwupd example firmware auto-upload.
License: GPLv2+
Url: https://github.com/redhat-et/firmware-on-the-edge

Source0: fwupd-auto-update.service

BuildRequires: systemd
Requires: fwupd

%{?systemd_requires}

%description

This package provides an auto-update example for fwupd based peripherals
for use with
https://github.com/redhat-et/firmware-on-the-edge/tree/main/firmware-examples

%prep

if [ -d  %{buildroot}%{firmwareStore} ]
then
   rm -rf  %{buildroot}%{firmwareStore}
fi

mkdir -p %{buildroot}%{firmwareStore}

%clean
rm -rf  %{buildroot}

%install

install -d -m755 %{buildroot}/%{_unitdir}
install -p -m644 %{SOURCE0} %{buildroot}%{_unitdir}/fwupd-auto-update.service


%post

%systemd_post fwupd-auto-update.service

%preun
%systemd_preun fwupd-auto-update.service

%postun


%files
%{_unitdir}/fwupd-auto-update.service


%changelog
* Wed Mar 22 2023 Miguel Angel Ajo Pelayo <majopela@redhat.com> . 2.1.0-2
fwupd auto update example
